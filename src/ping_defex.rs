use rand::{thread_rng, Rng};
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use std::env;

pub async fn maybe_ping() {
    let random_number = thread_rng().gen_range(0..19);
    if random_number == 14 {
        ping_user(325735841089454080, 1003043982621802598).await;
    }
}

pub async fn ping_user(user_id: u64, channel_id: u64) {
    let token = env::var("DTOKEN").expect("among");
    let http = Http::new(&token);
    let channel = ChannelId(channel_id);
    channel
        .send_message(&http, |m| m.content(format!("<@{}>", user_id)))
        .await
        .expect("couldnt send message");
}
