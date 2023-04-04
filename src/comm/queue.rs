use crate::uwuify::uwuify;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::{application_command::CommandDataOption, Interaction};
use serenity::prelude::Context;

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
        let queue_vec = handler.queue().current_queue();
        if queue_vec.is_empty() {
            return Some(uwuify("The queue is empty".to_string()));
        }
        let mut temp: String = String::new();
        let mut l = queue_vec.len();
        if l > 6 {
            l = 5
        }
        for i in 1..l {
            let title = match queue_vec[i].metadata().title.clone() {
                Some(title) => title,
                None => "unknown".to_string(),
            };
            temp = format!("{}\n{}", temp, title);
        }
        return Some(uwuify(format!("Next {} songs in queue:{}", l - 1, temp)));
    } else {
        return Some(uwuify("Not in voice".to_string()));
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("queue")
        .description("shows the next 5 tracks in the queue")
}
