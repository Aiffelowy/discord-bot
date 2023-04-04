use crate::uwuify::uwuify;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::{application_command::CommandDataOption, Interaction};
use serenity::prelude::Context;
use songbird::tracks::LoopState;
use std::time::Duration;

fn format_time(dur: Duration) -> String {
    let minutes = (dur.as_secs() / 60) % 60;
    let seconds = dur.as_secs() % 60;
    let hours = (dur.as_secs() / 3600) % 60;
    let mut time_string: String;

    if hours != 0 {
        time_string = hours.to_string();
        time_string.push(':');
        if minutes <= 9 {
            time_string.push('0');
        }
        time_string.push_str(&minutes.to_string());
    } else {
        time_string = minutes.to_string();
    }

    time_string.push(':');
    if seconds <= 9 {
        time_string.push('0');
    }
    time_string.push_str(&seconds.to_string());
    time_string
}

pub async fn run(
    _options: &[CommandDataOption],
    ctx: &Context,
    interaction: &Interaction,
) -> Option<String> {
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
        let handler = handler_lock.lock().await;
        let now_playing = match handler.queue().current() {
            Some(tracc) => tracc,
            None => return Some(uwuify("Nothing is playing!".to_string())),
        };
        let title = match now_playing.metadata().title.clone() {
            Some(t) => t,
            None => "unknown".to_string(),
        };
        let duration = format_time(match now_playing.metadata().duration.clone() {
            Some(d) => d,
            None => Duration::new(0, 0),
        });
        let url = match now_playing.metadata().source_url.clone() {
            Some(u) => u,
            None => "url unknown".to_string(),
        };
        let play_time = format_time(match now_playing.get_info().await {
            Ok(i) => i.play_time,
            Err(_) => Duration::new(0, 0),
        });
        let loops = match now_playing.get_info().await {
            Ok(i) => match i.loops {
                LoopState::Finite(number) => number.to_string(),
                LoopState::Infinite => "infinite".to_string(),
            },
            Err(_) => "unknown".to_string(),
        };

        let info: String = format!(
            "{}\nCurrently playing: {}\n{}/{}\t\t loops: {}",
            url,
            uwuify(title),
            play_time,
            duration,
            loops
        );
        return Some(info);
    } else {
        return Some("Not in voice".to_string());
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("now_playing")
        .description("shows info about currently playing track")
}
