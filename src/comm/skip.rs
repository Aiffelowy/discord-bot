use crate::uwuify::uwuify;
use serenity::prelude::Context;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::{application_command::CommandDataOption, Interaction};

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction) -> Option<String> {
    let guild = ctx.cache.guild(interaction.clone().application_command().unwrap().guild_id.unwrap()).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.expect("voice client placed in at init").clone();
    
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        match queue.skip() {
            Ok(_) => (),
            Err(e) => {
                println!("Error while skipping track: {}", e); return Some("There was an error while skipping the track".to_string());
            }
        }

        return Some(uwuify("Skipping the track".to_string()))

    } else {
        return Some(uwuify("Not in voice".to_string()));
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("skip").description("skips the currently playing track")
}
