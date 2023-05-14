use anyhow::anyhow;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::prelude::Context;

use moodle::Moodle;

use crate::moodle_stuff::accounts::AccountList;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("login")
        .description("Login to moodle")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("normal")
                .description("Login to moodle")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("url")
                        .description("Your schools moodle url: e.g. https://moodle.nibis.de/school")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("username")
                        .description("Your username")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("password")
                        .description("Your password")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("token")
                .description("Login to moodle")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("url")
                        .description("Your schools moodle url: e.g. https://moodle.nibis.de/school")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("name")
                        .description("A custom name to refer to this account in other commands")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("token")
                        .description("Your moodle mobile app token")
                        .kind(CommandOptionType::String)
                        .required(true)
                        .min_length(32)
                        .max_length(32)
                })
        })
}

pub async fn run(ctx: &Context, command: ApplicationCommandInteraction) {
    let guild_id = command
        .guild_id
        .expect("This command can only be run in guilds");

    let options = &command.data.options[0].options;

    let url = get_string(&options[0]).to_string();
    let name = get_string(&options[1]);

    let client = match command.data.options[0].name.as_ref() {
        "normal" => {
            let password = get_string(&options[2]);
            Moodle::new_with_login(url, name, password).await
        }
        "token" => {
            let token = get_string(&options[2]).to_string();
            Ok(Moodle::new_with_token(url, token))
        }
        _ => Err(anyhow!("Impossible login form")),
    };

    let mut account_list = AccountList::get_from_file(guild_id);

    match client {
        Ok(client) => match account_list.add_account(client, name).await {
            Ok(_) => send_response(&ctx, &command, "Successfully logged in!"),
            Err(why) => {
                println!("{:#?}", why);
                send_response(&ctx, &command, "Failed to fetch courses")
            }
        },
        Err(why) => {
            println!("{:#?}", why);
            send_response(&ctx, &command, "Failed to login to moodle")
        }
    }
    .await;
}

fn get_string(field: &CommandDataOption) -> &str {
    field
        .value
        .as_ref()
        .expect("Required field is empty")
        .as_str()
        .expect("Value not a string")
        .clone()
}

async fn send_response(ctx: &Context, command: &ApplicationCommandInteraction, message: &str) {
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| msg.content(message).ephemeral(true))
        })
        .await
    {
        println!("{}", why);
    }
}
