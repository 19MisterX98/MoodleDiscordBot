extern crate core;

use std::env;

use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::prelude::GatewayIntents;
use serenity::{async_trait, Client};
use tokio::spawn;

use crate::background::scan_scheduler::scan_continuous;

mod background;
mod commands;
mod moodle_stuff;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("is connected: {:?}", ready.user.name);

        let global_commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::course_selection::register(command))
                .create_application_command(|command| commands::login::register(command))
                .create_application_command(|command| commands::logout::register(command))
                .create_application_command(|command| commands::update::register(command))
        })
        .await;

        if let Err(why) = global_commands {
            println!("Failed to create global commands: {:#?}", why);
        }

        for guild in ready.guilds.iter() {
            let http = ctx.http.clone();
            let id = guild.id;
            spawn(async move { scan_continuous(id, http).await });
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "course-selection" => commands::course_selection::run(&ctx, command).await,
                "login" => commands::login::run(&ctx, command).await,
                "update" => commands::update::run(&ctx, command).await,
                "logout" => commands::logout::run(&ctx, command).await,
                _ => {}
            };
        } else if let Interaction::MessageComponent(component) = interaction {
            match component
                .data
                .custom_id
                .as_str()
                .split_whitespace()
                .next()
                .unwrap()
            {
                "courseselection" => {
                    commands::course_selection::save_new_selection(&ctx, component).await
                }
                "logout" => commands::logout::save_deletion(&ctx, component).await,
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected \"DISCORD_TOKEN\" Environment variable");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
