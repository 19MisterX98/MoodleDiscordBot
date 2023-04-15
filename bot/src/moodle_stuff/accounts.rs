use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::{ChannelId, GuildId};

use moodle::Moodle;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountList {
    guild: GuildId,
    accounts: HashMap<String, Moodle>, // (AccountName, ClientData)
    mapping: IndexMap<String, (i64, String, HashSet<ChannelId>)>, // CourseName, (Course_id, AccountName, Channels)
}

impl AccountList {
    fn new(guild_id: GuildId) -> AccountList {
        AccountList {
            guild: guild_id,
            accounts: HashMap::new(),
            mapping: IndexMap::new(),
        }
    }

    pub fn get_manuel_update_info(
        &self,
        channel_id: &ChannelId,
    ) -> Vec<(&str, i64, &Moodle, &HashSet<ChannelId>)> {
        let courses = self.get_active_courses_for_channel(channel_id);
        courses
            .iter()
            .map(|course| self.get_course_info(course))
            .collect()
    }

    fn get_active_courses_for_channel(&self, channel_id: &ChannelId) -> Vec<&String> {
        self.mapping
            .iter()
            .filter(|(_, (_, _, channels))| channels.contains(channel_id))
            .map(|(name, _)| name)
            .collect()
    }

    pub fn next_valid_course(
        &self,
        index: Arc<Mutex<usize>>,
    ) -> Option<(&str, i64, &Moodle, &HashSet<ChannelId>)> {
        if self.mapping.len() == 0 {
            // no courses registered
            return None;
        }

        let mut index = index.lock().unwrap();
        *index %= self.mapping.len();
        let start_index = *index;

        loop {
            *index %= self.mapping.len();

            let (name, _) = self.mapping.get_index(*index).unwrap();
            let info = self.get_course_info(name);
            *index += 1;

            if !info.3.is_empty() {
                return Some(info);
            }
            if *index == start_index {
                // no courses are mapped to channels
                return None;
            }
        }
    }

    fn get_course_info<'a, 'b>(
        &'a self,
        course: &'b str,
    ) -> (&'b str, i64, &'a Moodle, &'a HashSet<ChannelId>) {
        let (a, b, c) = self.mapping.get(course).unwrap();
        let b = self.accounts.get(b).unwrap();
        (course, *a, b, c)
    }

    /// updates courses too
    pub async fn get_course_map_for_channel(
        &mut self,
        channel_id: &ChannelId,
    ) -> Vec<(&String, bool)> {
        let mut up2date_courses = HashMap::new();
        for account in self.accounts.iter() {
            let acc_courses: HashMap<String, (i64, &String)> = account
                .1
                .get_courses(None)
                .await
                .unwrap_or_default()
                .courses
                .into_iter()
                .map(|course| (course.fullname, (course.id, account.0)))
                .collect();
            up2date_courses.extend(acc_courses);
        }

        if !up2date_courses.is_empty() {
            self.mapping
                .retain(|course, _| up2date_courses.contains_key(course));
            for (course, (course_id, account)) in up2date_courses {
                self.mapping
                    .entry(course)
                    .or_insert((course_id, account.into(), HashSet::new()));
            }
        }

        let result = self
            .mapping
            .iter()
            .map(|(course, (_, _, channels))| {
                let active = channels.contains(channel_id);
                (course, active)
            })
            .collect();
        self.save_to_file();
        result
    }

    pub fn set_course_map_for_channel(
        &mut self,
        channel_id: &ChannelId,
        selection: &Vec<String>,
        options: &Vec<&String>,
    ) {
        for (course_name, (_, _, channels)) in self.mapping.iter_mut() {
            if selection.contains(course_name) {
                channels.insert(channel_id.into());
            } else if options.contains(&course_name) {
                channels.remove(channel_id);
            }
        }
        self.save_to_file();
    }

    pub async fn add_account(&mut self, account: Moodle, name: &str) -> Result<(), Box<dyn Error>> {
        account.get_courses(None).await?;
        self.accounts.insert(name.into(), account);
        self.save_to_file();
        Ok(())
    }

    pub fn get_accounts(&self) -> Vec<&String> {
        self.accounts.keys().collect()
    }

    pub fn remove_account(&mut self, name: &str) {
        self.accounts.remove(name);

        self.mapping.retain(|_, (_, acc_name, _)| name != acc_name);
        self.save_to_file();
    }

    fn save_to_file(&self) {
        let file_path = format!("data/{}/accounts.json", &self.guild);
        let file_path = Path::new(&file_path);
        fs::create_dir_all(&file_path.parent().unwrap()).unwrap();

        if let Ok(account_list) = serde_json::to_string(&self) {
            let byte_array = account_list.as_bytes();
            if let Ok(mut file) = File::create(&file_path) {
                if let Err(why) = file.write(byte_array) {
                    println!("Couldnt save to file {why}");
                }
            }
        }
    }

    pub fn get_from_file(guild_id: GuildId) -> AccountList {
        let file_path = format!("data/{}/accounts.json", &guild_id);
        let file_path = Path::new(&file_path);
        fs::create_dir_all(&file_path.parent().unwrap()).unwrap();

        match fs::read_to_string(&file_path) {
            Ok(accounts) => serde_json::from_str::<AccountList>(&accounts).unwrap(),
            Err(_) => AccountList::new(guild_id),
        }
    }
}
