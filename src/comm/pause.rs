use crate::uwuify::uwuify;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::{application_command::CommandDataOption, Interaction};
use serenity::prelude::Context;
use songbird::tracks::PlayMode;

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
        let queue = handler.queue().current_queue();
        if queue.len() == 0 {
            return Some(uwuify("Nothing is playing!".to_string()));
        }
        let track = &handler.queue().current_queue()[0];
        match track.get_info().await.unwrap().playing {
            PlayMode::Play => match track.pause() {
                Ok(_) => return Some(uwuify("Paused ze tracc".to_string())),
                Err(e) => return Some(format!("Coulnt pause ze tracc: {}", e)),
            },
            PlayMode::Pause => match track.play() {
                Ok(_) => return Some(uwuify("Resumed ze tracc".to_string())),
                Err(e) => return Some(format!("Couldnt resume ze tracc: {}", e)),
            },
            PlayMode::Stop => {
                return Some(uwuify(
                    "cannot pause/unpause: ze tracc has beed stopped".to_string(),
                ))
            }
            PlayMode::End => {
                return Some(uwuify(
                    "cannot pause/unpause: ze tracc has ended".to_string(),
                ))
            }
            _ => return Some("HOOWW".to_string()),
        }
    } else {
        return Some(uwuify("Not in voice".to_string()));
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("pause")
        .description("pauses/resumes currently playing tracc")
}
