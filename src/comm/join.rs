use crate::{uwuify::uwuify, voice_recog};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::{application_command::CommandDataOption, Interaction};
use serenity::prelude::Context;
use songbird::CoreEvent;

use voice_recog::model_test;

pub async fn run(
    _options: &[CommandDataOption],
    ctx: &Context,
    interaction: &Interaction,
) -> Option<String> {
    let current_user_id = ctx.cache.current_user().id;
    let intx = interaction.clone().application_command().unwrap();
    let user_id = intx.member.unwrap().user.id;
    let guild = ctx.cache.guild(intx.guild_id.unwrap()).unwrap();
    let guild_id = guild.id;

    let check_if_in_voice = guild
        .voice_states
        .get(&current_user_id)
        .and_then(|voice_state| voice_state.channel_id);
    if check_if_in_voice.is_some() {
        return None;
    }

    let channel_id = guild
        .voice_states
        .get(&user_id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Some(uwuify("You're not in a voice channel >~<".to_string()));
        }
    };
    let manager = songbird::get(ctx)
        .await
        .expect("voice client placed in a init.")
        .clone();
    let (handler_lock, c_result) = manager.join(guild_id, connect_to).await;
    if let Ok(_) = c_result {
        let mut handler = handler_lock.lock().await;
        let receiver = model_test::Destroyer::new();
        handler.remove_all_global_events();
        handler.add_global_event(
            CoreEvent::SpeakingStateUpdate.into(),
            receiver
            //model_test::Test::new(ctx.clone(), guild_id),
        );
        //handler.add_global_event(CoreEvent::SpeakingUpdate.into(), Receiver::new());
        handler.add_global_event(
            CoreEvent::VoicePacket.into(),
            receiver
            //model_test::ReceiverWithModel::new(ctx.clone(), guild_id),
        );
        handler.add_global_event(
            CoreEvent::DriverDisconnect.into(),
            model_test::Destroyer::new(),
        )
        //handler.add_global_event(CoreEvent::RtcpPacket.into(), Receiver::new());
        //handler.add_global_event(CoreEvent::ClientDisconnect.into(), Receiver::new());
    }

    Some(uwuify("Joined channel".to_string()))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("join")
        .description("Tell waifu to join your voice channel")
}
