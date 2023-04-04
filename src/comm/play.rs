use crate::uwuify::uwuify;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::Interaction;
use serenity::prelude::Context;
use songbird::input::Input;

pub async fn run(
    options: &[CommandDataOption],
    ctx: &Context,
    interaction: &Interaction,
) -> Option<String> {
    match super::join::run(options, ctx, interaction).await {
        Some(_) => (),
        None => (),
    }
    let mut is_search: bool = false;
    let option = options
        .get(0)
        .expect("Expected URL")
        .resolved
        .as_ref()
        .expect("Expected String");

    let url: String;

    if let CommandDataOptionValue::String(ur) = option {
        url = ur.clone()
    } else {
        return Some(uwuify("Please provide a valid user".to_string()));
    }

    if !url.starts_with("https://") && !url.starts_with("http://") {
        //return Some(uwuify("Must provide a valid url".to_string()));
        is_search = true;
    }
    if url.contains("Zy4z6Bxp4LI")
        || (url.contains("lead")
            || url.contains("pipe")
            || url.contains("metal")
            || url.contains("loudest"))
    {
        return Some(uwuify(
            "why dont youwu metal pipe yourself some bitches?".to_string(),
        ));
    } else if url.contains("cdn") {
        return Some(uwuify(
            "why dont youwu content delivery network yourself some bitches?".to_string(),
        ));
    } else if url.contains("discordapp") {
        return Some(uwuify("DZISAS STOP IT GOD DAMMIT!!".to_string()));
    }

    let guild = ctx
        .cache
        .guild(
            interaction
                .clone()
                .application_command()
                .unwrap()
                .guild_id
                .unwrap(),
        )
        .unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("voice client placed in at init")
        .clone();
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let source;
        if is_search {
            source = match songbird::input::restartable::Restartable::ytdl_search(url, true).await {
                Ok(source) => source,
                Err(e) => {
                    eprintln!("Error searching for source: {}", e);
                    return Some(uwuify("couldnt search for da sauce".to_string()));
                }
            }
        } else {
            source = match songbird::input::restartable::Restartable::ytdl(url, true).await {
                Ok(source) => source,
                Err(e) => {
                    eprintln!("Error starting source: {}", e);
                    return Some("Error sourcing ffmpeg".to_string());
                }
            };
        }
        let source_norm: songbird::input::Input = Input::from(source);
        let title = match source_norm.metadata.title.clone() {
            Some(title) => title,
            None => "unknown".to_string(),
        };
        let channel = match source_norm.metadata.channel.clone() {
            Some(c) => c,
            None => "unknown".to_string(),
        };
        if channel.contains("Dart Frog") && !(title.contains("ocoded")) {
            return Some(uwuify("Seriously, fock off Dart Frog!!".to_string()));
        }
        println!("{} {}", title, channel);
        handler.enqueue_source(source_norm);

        return Some(uwuify(format!("Added {} to queue", title)));
    } else {
        return Some(uwuify("You're not in a voice >~<".to_string()));
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play a youtube video")
        .create_option(|option| {
            option
                .name("query")
                .description("youtube url or search query")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
