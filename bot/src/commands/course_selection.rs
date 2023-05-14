use serenity::builder::{CreateApplicationCommand, CreateSelectMenu};
use serenity::model::application::component::ActionRowComponent;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

use crate::moodle_stuff::accounts::AccountList;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("course-selection")
        .description("Select/Deselect moodle courses attached to this channel")
        .dm_permission(false)
}

pub async fn run(ctx: &Context, command: ApplicationCommandInteraction) {
    let guild_id = command
        .guild_id
        .expect("This command can only be run in guilds");
    let channel_id = command.channel_id;
    let mut accounts = AccountList::get_from_file(guild_id);
    let course_map = accounts.get_course_map_for_channel(&channel_id).await;

    if course_map.is_empty() {
        let res = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.content(
                            "No courses found, make sure that you login first (/login normal)",
                        )
                    })
            })
            .await;

        if let Err(why) = res {
            println!("{:#?}", why);
        }
        return;
    }

    let res = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    msg.content("Select moodle course subscriptions for this channel")
                        .ephemeral(true)
                        .components(|components| {
                            for (row, chunk) in course_map.chunks(25).enumerate() {
                                components.create_action_row(|actions| {
                                    actions.create_select_menu(|menu| {
                                        create_moodle_course_selection(chunk, menu, row)
                                    })
                                });
                            }
                            components
                        })
                })
        })
        .await;

    if let Err(why) = res {
        println!("{:#?}", why);
    }
}

fn create_moodle_course_selection<'a>(
    course_map: &[(&String, bool)],
    menu: &'a mut CreateSelectMenu,
    row: usize,
) -> &'a mut CreateSelectMenu {
    menu.min_values(0)
        .max_values(course_map.len().min(25) as u64)
        .custom_id(format!("courseselection {}", row));

    menu.options(|options| {
        course_map.iter().for_each(|(course, active)| {
            options.create_option(|option| {
                option
                    .value(course)
                    .label(course)
                    .default_selection(*active)
            });
        });
        options
    })
}

pub async fn save_new_selection(ctx: &Context, component: MessageComponentInteraction) {
    let guild_id = component
        .guild_id
        .expect("This command can only be run in guilds");
    let channel_id = component.channel_id;
    let mut accounts = AccountList::get_from_file(guild_id);
    let selection = &component.data.values;
    let row = &component.data.custom_id.split_whitespace().nth(1).unwrap();
    let row: usize = row.parse().unwrap();
    if let ActionRowComponent::SelectMenu(menu) = &component.message.components[row].components[0] {
        let options: Vec<_> = menu.options.iter().map(|option| &option.value).collect();

        accounts.set_course_map_for_channel(&channel_id, &selection, &options);

        let response = component
            .create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::DeferredUpdateMessage)
            })
            .await;
        if let Err(why) = response {
            println!("{:#?}", why);
        }
    };
}
