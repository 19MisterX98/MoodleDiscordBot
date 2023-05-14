use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::Context;

use crate::moodle_stuff::accounts::AccountList;
use crate::moodle_stuff::course_scanning::update_course;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("update")
        .description("Scans for updates in all moodle courses linked to this channel")
        .dm_permission(false)
}

pub async fn run(ctx: &Context, command: ApplicationCommandInteraction) {
    let channel = command.channel_id;
    let guild_id = command
        .guild_id
        .expect("This command can only be run in guilds");

    let res = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|x| x.ephemeral(true))
        })
        .await;
    if let Err(why) = res {
        println!("{}", why);
    }

    let account_list = AccountList::get_from_file(guild_id);
    let info = account_list.get_manuel_update_info(&channel);

    for (name, id, client, channels) in info {
        update_course(name, id, channels, &ctx.http, guild_id, client).await;
    }

    let res = command
        .edit_original_interaction_response(&ctx.http, |response| response.content("Finished"))
        .await;
    if let Err(why) = res {
        println!("{}", why);
    }
}
