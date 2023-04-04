use crate::uwuify::uwuify;
use serenity::prelude::Context;
use songbird::tracks::LoopState;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::{application_command::{CommandDataOption, CommandDataOptionValue}, Interaction};

pub async fn run(options: &[CommandDataOption], ctx: &Context, interaction: &Interaction) -> Option<String> {
    let guild = ctx.cache.guild(interaction.clone().application_command().unwrap().guild_id.unwrap()).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.expect("voice client placed in at init").clone();
    
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let option = match options.get(0) {
            Some(o) => o.resolved.as_ref().expect("sussy"),
            None => &CommandDataOptionValue::Integer(0),
        };
        let playing = &handler.queue().current_queue()[0];
        let if_loop = match playing.get_info().await {
            Ok(i) => match i.loops {
                LoopState::Finite(number) => number,
                LoopState::Infinite => 1,
            }
            Err(_) => 1,
        };
        
        if if_loop != 0 {
            match playing.disable_loop() {
                Ok(_) => return Some(uwuify("No longer looping teh track".to_string())),
                Err(e) => {println!("Error while looping: {}", e); return Some(format!("Error while looping: {}", e));}
            }
        }

        if let CommandDataOptionValue::Integer(num) = option {
            let number = num.clone();
            if number == 0 {
                match playing.enable_loop() {
                    Ok(_) => return Some(uwuify("Looping the track indefinitely".to_string())),
                    Err(e) => {println!("Error while looping: {}", e); return Some(format!("Error while looping: {}", e))},
                };
            }
            match playing.loop_for(match number.try_into() {
                Ok(n) => n,
                Err(_) => usize::MAX,
            }) {
                Ok(_) => return Some(uwuify(format!("Looping the track {} times", num))),
                Err(e) => {println!("Error while looping: {}", e); return Some(format!("Error while looping: {}", e))},
            };
        }
        return Some("idk how u did this but ok".to_string());
    } else {
        return Some(uwuify("Not in voice".to_string()));
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("loop").description("Toggle looping of currently playing track indefinitely or a specified amount of times").create_option(|option| {
        option
            .name("number")
            .description("number of times to loop")
            .kind(CommandOptionType::Integer)
            .required(false)
    })
}
