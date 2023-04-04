use clokwerk::{AsyncScheduler, Interval};
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::prelude::Message;
use serenity::model::{
    application::command::Command,
    application::interaction::{Interaction, InteractionResponseType},
    gateway::Ready,
};
use serenity::prelude::GatewayIntents;
use songbird::driver::DecodeMode;
use songbird::{Config, SerenityInit};
use std::env;
use std::time::Duration;

mod comm;
mod ping_defex;
mod responses;
mod reqwest_audio_data;
mod uwuify;
mod wav;

mod voice_recog;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction.clone() {
            let content = match command.data.name.as_str() {
                "join" => match comm::join::run(&command.data.options, &ctx, &interaction).await {
                    Some(res) => res,
                    None => "already in a channel!".to_string(),
                },
                "ping" => comm::ping::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                "leave" => comm::leave::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                "play" => "fetching the video".to_string(),
                "skip" => comm::skip::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                "queue" => comm::queue::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                "now_playing" => comm::now_playing::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                "loop" => comm::loop_among::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                "say" => "AI-ifying youw wequest".to_string(),
                "pause" => comm::pause::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                _ => "not implemented >~<".to_string(),
            };

            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", e);
            };

            let edit_content = match command.data.name.as_str() {
                "say" => comm::say::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                "play" => comm::play::run(&command.data.options, &ctx, &interaction)
                    .await
                    .unwrap(),
                _ => "".to_string(),
            };
            if !edit_content.is_empty() {
                if let Err(e) = command
                    .edit_original_interaction_response(&ctx.http, |response| {
                        response.content(edit_content)
                    })
                    .await
                {
                    println!("Cannot edit original response: {}", e);
                };
            }
        }
    }
    async fn message(&self, ctx: Context, mut msg: Message) {
        //println!("{}", msg.content);
        if msg.author.id.as_u64() == &(1056729359764963421 as u64)
            && (msg.mentions_user_id(457180041575661568)
                || msg
                    .content
                    .contains("https://tenor.com/view/so-uncivilised-gif-24177115")
                || match msg.attachments.pop() {
                    Some(att) => att.filename.contains("kenobi"),
                    None => false,
                })
        {
            //msg.reply(&ctx.http, responses::SHUT_UP_RESPONSES[rng_number]).await.expect("couldn't send a message");
            let channel_id = msg.channel_id;
            let rand_file = responses::random_response();
            let file = rand_file.as_path();
            channel_id
                .send_message(&ctx.http, |m| m.reference_message(&msg).add_file(file))
                .await
                .expect("couldnt send shut up message");
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in as {} UwU\n", ready.user.name);

        Command::create_global_application_command(&ctx.http, |command| {
            comm::join::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::leave::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::play::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::skip::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::queue::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::now_playing::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::loop_among::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::say::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::ping::register(command)
        })
        .await
        .expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            comm::pause::register(command)
        })
        .await
        .expect("couldn't create command");
        let mut scheduler = schedule_ping(clokwerk::Interval::Hours(2)).await;
        tokio::spawn(async move {
            loop {
                scheduler.run_pending().await;
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        });
    }
}

async fn schedule_ping(interval: Interval) -> clokwerk::AsyncScheduler {
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(interval).run(|| async {
        ping_defex::maybe_ping().await;
    });
    scheduler
}

#[tokio::main]
async fn main() {
    let token = env::var("DTOKEN").expect("token");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let songbird_config = Config::default().decode_mode(DecodeMode::Decode);
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .register_songbird_from_config(songbird_config)
        .await
        .expect("The was an error");

    if let Err(err) = client.start().await {
        println!("Client error: {}", err);
    }
}
/*
fn check_msg(result: SerenityResult<Message>) {
    if let Err(e) = result {
        println!("Error sending message: {}", e);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "My ballz run on Rust! (this time for real UwU)").await.unwrap();

    Ok(())
}
*/
