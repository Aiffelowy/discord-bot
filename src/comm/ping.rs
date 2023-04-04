use serenity::builder::CreateApplicationCommand;
use serenity::prelude::Context;
use serenity::model::prelude::interaction::{Interaction, application_command::CommandDataOption};

pub async fn run(_options: &[CommandDataOption], _ctx: &Context, _interaction: &Interaction) -> Option<String> {
    Some("https://tenor.com/view/vtuber-saruei-gay-gif-23058982".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("test. if the bot doesn't respond to this command then its fucked and i wanna die")
}
