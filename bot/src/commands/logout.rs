use serenity::builder::{CreateApplicationCommand, CreateSelectMenu};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

use crate::moodle_stuff::accounts::AccountList;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("logout")
        .description("Returns a list where you can select an account to log out of")
}

pub async fn run(ctx: &Context, command: ApplicationCommandInteraction) {
    let guild_id = command
        .guild_id
        .expect("This command can only be run in guilds");
    let accounts = AccountList::get_from_file(guild_id);
    let account_list = accounts.get_accounts();

    if account_list.is_empty() {
        let res = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.content(
                            "No accounts found, make sure that you login first (/login normal)",
                        )
                        .ephemeral(true)
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
                .kind(ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    msg.content("Select a moodle account to log out of")
                        .ephemeral(true)
                        .components(|components| {
                            components.create_action_row(|actions| {
                                actions.create_select_menu(|menu| {
                                    create_account_selection(account_list, menu)
                                })
                            })
                        })
                })
        })
        .await;

    if let Err(why) = res {
        println!("{:#?}", why);
    }
}

fn create_account_selection<'a>(
    accounts: Vec<&String>,
    menu: &'a mut CreateSelectMenu,
) -> &'a mut CreateSelectMenu {
    menu.custom_id("logout");
    menu.options(|options| {
        for account in accounts {
            options.create_option(|option| option.value(account).label(account));
        }
        options
    })
}

pub async fn save_deletion(ctx: &Context, component: MessageComponentInteraction) {
    let guild_id = component
        .guild_id
        .expect("This command can only be run in guilds");
    let mut accounts = AccountList::get_from_file(guild_id);

    accounts.remove_account(&component.data.values.first().unwrap());

    let response = component
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await;
    if let Err(why) = response {
        println!("{:#?}", why);
    }
}
