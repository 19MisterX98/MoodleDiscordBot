use std::borrow::Cow;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use indexmap::IndexMap;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use serenity::model::prelude::{AttachmentType, GuildId};
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use moodle::data::comparable::{compare, diff_module_entries, ModuleEntry};
use moodle::data::course_traversal::get_course_info;
use moodle::data::gen_module::GenModule;
use moodle::Moodle;

/// Represents a moodle course as a discord message
struct MoodleEmbed {
    mod_icon_url: String,
    files: Vec<(String, Vec<u8>)>,
    color: u32,
    description: String,
    name: String,
    link: Option<String>,
}

impl MoodleEmbed {
    fn new(color: u32, mod_icon_url: String, name: String, link: Option<String>) -> MoodleEmbed {
        MoodleEmbed {
            mod_icon_url,
            files: vec![],
            color,
            description: "".into(),
            name,
            link,
        }
    }

    /// Create a discord embed that represents a new moodle module
    async fn added(module: GenModule, client: &Moodle) -> MoodleEmbed {
        let mut embed = MoodleEmbed::new(0x00FF00, module.mod_icon_url, module.name, module.link);
        embed.add_raw_entries(module.entries);
        embed.add_files(module.files, client).await;
        embed
    }

    fn add_raw_entries(&mut self, entries: IndexMap<String, String>) {
        for (key, val) in entries {
            self.add_entry(format!("**{key}:** {val}"));
        }
    }

    /// Create a discord embed that represents a removed moodle module
    fn removed(module: GenModule) -> MoodleEmbed {
        let mut embed = MoodleEmbed::new(0xFF0000, module.mod_icon_url, module.name, module.link);
        embed.add_raw_entries(module.entries);
        for (key, _) in module.files {
            embed.add_entry(format!("__File:__ {key}"));
        }
        embed
    }

    /// download files
    async fn add_files(&mut self, files: IndexMap<String, String>, client: &Moodle) {
        for (name, url) in files {
            // download files under 8mb (max file size on discord)
            let file = client.download_file(url, 8000000).await;
            if let Ok(file) = file {
                self.files.push((name, file));
            } else {
                self.add_entry(format!("__Added large file:__ {name}"));
            }
        }
    }

    /// Create a discord embed that represents a modified moodle module
    async fn changed(
        module_old: GenModule,
        module_new: GenModule,
        client: &Moodle,
    ) -> Option<MoodleEmbed> {
        let mut embed = MoodleEmbed::new(
            0x0000FF,
            module_new.mod_icon_url,
            module_new.name,
            module_new.link,
        );
        let description = diff_module_entries(module_old.entries, module_new.entries);
        // representation of module entries
        for (key, val) in description {
            let entry = match val {
                ModuleEntry::Added(val) => format!("🟢 **{key}:** {val}"),
                ModuleEntry::Removed(val) => format!("🔴 **{key}:** {val}"),
                ModuleEntry::Changed(old, new) => format!("🔵**{key}**\n__From:__ {old}\n__To:__ {new}"),
            };
            embed.add_entry(entry);
        }
        // add normal module entries for removed files and store urls for new files
        let files: IndexMap<_, _> = diff_module_entries(module_old.files, module_new.files)
            .into_iter()
            .flat_map(|(name, val)| match val {
                ModuleEntry::Added(file) => Some((name, file)),
                ModuleEntry::Removed(_) => {
                    embed.add_entry(format!("__Removed file:__ {name}"));
                    None
                }
                ModuleEntry::Changed(_, file) => {
                    embed.add_entry(format!("__Removed file:__ {name}"));
                    Some((name, file))
                }
            })
            .collect();
        // download new files
        embed.add_files(files, client).await;
        // only return a module if anything changed
        if embed.files.is_empty() && embed.description.is_empty() {
            None
        } else {
            Some(embed)
        }
    }

    fn add_entry(&mut self, string: String) -> bool {
        // an entry value is capped at 1500 chars
        let slice = string.chars().take(1500);
        // if the embed is larger than 4000 chars then dont add new entries (discord limit)
        let len = self.len() + slice.clone().count();
        if len > 4000 {
            return false;
        }
        // new line if we append to
        if !self.description.is_empty() {
            self.description.push('\n');
        }
        self.description.extend(slice);
        true
    }

    fn len(&self) -> usize {
        self.description.chars().count()
    }
}

async fn get_changes(
    client: &Moodle,
    course_id: i64,
    course_name: &str,
    guild_id: GuildId,
) -> anyhow::Result<Vec<MoodleEmbed>> {
    let new_course = get_course_info(client, course_id).await?;
    //save the the new state of the course and read the old one
    let old_course = read_old_write_new_file(&new_course, &course_name, &guild_id).await?;
    //figure out which modules are new, old or changed
    let mapped_modules = compare(old_course, new_course);

    let mut embeds = vec![];
    for module in mapped_modules.a {
        let embed = MoodleEmbed::removed(module);
        embeds.push(embed)
    }
    for module in mapped_modules.b {
        let embed = MoodleEmbed::added(module, &client).await;
        embeds.push(embed);
    }
    for (old_module, new_module) in mapped_modules.common {
        if let Some(embed) = MoodleEmbed::changed(old_module, new_module, &client).await {
            embeds.push(embed);
        }
    }

    Ok(embeds)
}

async fn read_old_write_new_file(
    new_course: &Vec<GenModule>,
    course_name: &str,
    guild: &GuildId,
) -> anyhow::Result<Vec<GenModule>> {
    // read old course state
    let file_path = format!("courses/{}/{}.json", guild, course_name);
    let file_path = Path::new(&file_path);
    fs::create_dir_all(file_path.parent().unwrap()).await?;
    let old_data = fs::read_to_string(&file_path).await;
    let old_data = match old_data {
        Ok(old_data) => serde_json::from_str::<Vec<GenModule>>(&old_data)?,
        Err(_) => vec![],
    };
    //write new course state
    if let Ok(new_course_string) = serde_json::to_string(new_course) {
        let byte_array = new_course_string.as_bytes();
        if let Ok(mut file) = File::create(&file_path).await {
            if let Err(why) = file.write(byte_array).await {
                println!("Couldnt write to file {why}");
            }
        }
    }

    Ok(old_data)
}

async fn send_changes(
    embeds: Vec<MoodleEmbed>,
    course_name: &str,
    channels: &HashSet<ChannelId>,
    http: &Arc<Http>,
) {
    for moodle_embed in embeds {
        for channel in channels {
            if let Err(why) = channel
                .send_message(http, |message| {
                    message.add_embed(|embed| {
                        embed
                            .color(moodle_embed.color)
                            .title(&moodle_embed.name)
                            .footer(|a| a.text(&course_name))
                            .description(&moodle_embed.description)
                            .thumbnail(&moodle_embed.mod_icon_url);
                        if let Some(link) = &moodle_embed.link {
                            embed.url(link);
                        }
                        embed
                    });

                    for (name, data) in moodle_embed.files.iter() {
                        message.add_file(AttachmentType::Bytes {
                            data: Cow::from(data),
                            filename: name.clone(),
                        });
                    }
                    message
                })
                .await
            {
                println!("channel: {}, course: {}, {:#?}", channel, course_name, why);
            }
        }
    }
}

pub async fn update_course(
    course_name: &str,
    course_id: i64,
    channels: &HashSet<ChannelId>,
    http: &Arc<Http>,
    guild: GuildId,
    client: &Moodle,
) {
    let embeds = get_changes(client, course_id, course_name, guild).await;
    match embeds {
        Ok(embeds) => {
            send_changes(embeds, course_name, channels, http).await;
        }
        Err(why) => {
            println!("Failed to update course: {:?}", why);
        }
    }
}
