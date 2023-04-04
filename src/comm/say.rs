use crate::uwuify::uwuify;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::Interaction;
use serenity::prelude::Context;

use songbird::input::Codec;
use songbird::input::Container;
use songbird::input::Metadata;
use songbird::input::Reader;
use std::fs::File;
use std::process::Command;

pub async fn run(
    options: &[CommandDataOption],
    ctx: &Context,
    interaction: &Interaction,
) -> Option<String> {
    super::join::run(options, ctx, interaction)
        .await
        .unwrap_or_default();

    let option = options
        .get(0)
        .expect("Expected String")
        .resolved
        .as_ref()
        .expect("Expected String");

    let mut str_to_say: String;

    if let CommandDataOptionValue::String(s) = option {
        str_to_say = s.clone()
    } else {
        return Some(uwuify("Please provide a valid user".to_string()));
    }
    if str_to_say.contains("wwww") {
        return Some(uwuify(
            "why dont you wwww yourself some bitches, also fock off".to_string(),
        ));
    }
    str_to_say = str_to_say
        .replace("/", "")
        .replace("\"", "")
        .replace("\'", "")
        .replace("\\", "");
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
        let audio_data = match crate::reqwest_audio_data::get_audio_data(str_to_say).await {
            Ok(data) => data,
            Err(_) => return Some(uwuify("The AI is not available right now :(".to_string())),
        };
        match crate::wav::to_wav(audio_data) {
            Ok(()) => (),
            Err(_) => return Some("couldn't generate wav".to_string()),
        };
        //ffprobe stoopidness
        let ffprobe_command = Command::new("ffprobe")
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                "among.wav",
            ])
            .output()
            .expect("among please no");
        let ffprobe_out = std::str::from_utf8(ffprobe_command.stdout.as_slice()).unwrap();
        let ffprobe_json = serde_json::from_str(ffprobe_out).expect("couldn't parse json");
        //load ze file
        let file = File::open("among.wav").expect("failed to load a file");

        //prepere source
        let reader = Reader::from_file(file);
        let metadata: Metadata = Metadata::from_ffprobe_json(&ffprobe_json);
        println!(
            "channels: {:?}\n sample_rate: {:?}\n duration: {:?}\n",
            metadata.channels, metadata.sample_rate, metadata.duration
        );

        let source = songbird::input::Input::new(
            false,
            reader,
            Codec::FloatPcm,
            Container::Raw,
            Some(metadata),
        );

        handler.play_source(source);

        return Some(uwuify("saying the thing".to_string()));
    } else {
        return Some(uwuify("You're not in a voice >~<".to_string()));
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("say")
        .description("say a given string in voice chat")
        .create_option(|option| {
            option
                .name("string")
                .description("string to say")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
