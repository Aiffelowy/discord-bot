use crate::uwuify::uwuify;
use serenity::prelude::Context;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::{application_command::CommandDataOption, Interaction};


pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction) -> Option<String> {
    let guild = ctx.cache.guild(interaction.clone().application_command().unwrap().guild_id.unwrap()).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.expect("voice client placed in at init").clone();
    let has_handler = manager.get(guild_id).is_some();
    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            return Some(uwuify(format!("Failed: {}", e)));
        }
        return Some(uwuify("Left the channel".to_string()));
    } else {
        return Some(uwuify("Not in a voice".to_string()));
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leave").description("among my ballz")
}
