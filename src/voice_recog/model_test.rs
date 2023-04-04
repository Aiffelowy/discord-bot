use coqui_stt::Model;
use serenity::async_trait;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use songbird::model::payload::Speaking;
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};
use std::sync::Mutex;

// its 2 am, i dont care, i will rewrite this whole file later.
//
// ok i found a not-war-crime way of doing this and i will change the ai to whisper
static FUCK_ME_I_GIVE_UP: Mutex<Vec<Mutex<User>>> = Mutex::new(vec![]);

fn get_stream() -> Model {
    let mut model = Model::new("model.tflite").expect("couldnt load model");
    model
        .enable_external_scorer("among.scorer")
        .expect("couldnt load scorer");
    model
}

fn downsample(data: Vec<i16>) -> Vec<i16> {
    let mut new_vec: Vec<i16> = vec![];
    for i in data.iter().step_by(6) {
        new_vec.push(*i);
    }
    new_vec
}

fn process_audio(receiver: &ReceiverWithModel, audio: Vec<i16>) -> String {
    let processed_audio = receiver
        .stream
        .lock()
        .unwrap()
        .speech_to_text(&audio)
        .unwrap();
    processed_audio
}

async fn play_audio(ctx: &Context, guild_id: GuildId, filename: &str) {
    let manager = songbird::get(ctx).await.expect("fuck me i guess").clone();
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let source = songbird::ffmpeg(filename).await.unwrap();
        handler.enqueue_source(source);
        handler.queue().modify_queue(|queue| {
            queue.front().unwrap().pause().unwrap();
            let despacito = queue.pop_back().unwrap();
            queue.push_front(despacito);
            queue.front().unwrap().play().unwrap();
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
struct User {
    ssrc: u32,
    username: String,
    audio_data: Vec<i16>,
    num_of_packets: u32,
}

impl User {
    pub fn new(username: String, ssrc: u32) -> Self {
        Self {
            ssrc: ssrc,
            username: username,
            audio_data: vec![],
            num_of_packets: 0,
        }
    }
    pub fn add_audio(&mut self, audio: &mut Vec<i16>) {
        self.audio_data.append(audio);
        self.num_of_packets += 1;
    }
}
#[derive(Clone, Copy)]
pub struct Destroyer {}

impl Destroyer {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl VoiceEventHandler for Destroyer {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        println!("Disconnected!");
        *FUCK_ME_I_GIVE_UP.lock().unwrap() = vec![];
        None
    }
}

pub struct Test {
    ctx: Context,
    guild_id: GuildId,
}
impl Test {
    pub fn new(ctx: Context, guildid: GuildId) -> Self {
        Self {
            ctx: ctx,
            guild_id: guildid,
        }
    }
}
#[async_trait]
impl VoiceEventHandler for Test {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        use EventContext as Ctx;
        match ctx {
            Ctx::SpeakingStateUpdate(Speaking { ssrc, user_id, .. }) => {
                //println!("{:?}, {:?}", user_id, ssrc);
                let userid = user_id.unwrap().0;
                let username = self
                    .ctx
                    .cache
                    .member(self.guild_id, userid)
                    .unwrap()
                    .user
                    .name;
                for user in FUCK_ME_I_GIVE_UP.lock().unwrap().iter() {
                    if &user.lock().unwrap().ssrc == ssrc {
                        println!("{} already exists!", username);
                        return None
                    }
                }
                println!("Added {}!", username);
                FUCK_ME_I_GIVE_UP
                    .lock()
                    .unwrap()
                    .push(Mutex::new(User::new(username, ssrc.clone())));

            }
            _ => unimplemented!(),
        }
        None
    }
}
pub struct ReceiverWithModel {
    stream: Mutex<Model>,
    ctx: Context,
    guild_id: GuildId,
}
impl ReceiverWithModel {
    pub fn new(cont: Context, guild_id: GuildId) -> Self {
        Self {
            stream: Mutex::new(get_stream()),
            ctx: cont,
            guild_id: guild_id,
        }
    }
}

#[async_trait]
impl VoiceEventHandler for ReceiverWithModel {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        use EventContext as Ctx;
        match ctx {
            Ctx::VoicePacket(data) => {
                if let Some(audio) = data.audio {
                    let mut audio_cloned = downsample(audio.clone());
                    let mut processed_audio: String = String::new();
                    for user in FUCK_ME_I_GIVE_UP.lock().unwrap().iter() {
                        let user_cloned = user.lock().unwrap().clone();

                        if data.packet.ssrc == user_cloned.ssrc {
                            user.lock().unwrap().add_audio(&mut audio_cloned);
                        } else {
                            continue;
                        }

                        if user_cloned.num_of_packets >= 200 {
                            let audio = user_cloned.audio_data.clone();
                            processed_audio = std::thread::scope(|scope| scope.spawn(|| process_audio(&self, audio)).join().unwrap());
                            user.lock().unwrap().audio_data = vec![];
                            user.lock().unwrap().num_of_packets = 0;
                            println!("{}: {}", user_cloned.username, processed_audio);
                        }
                    }
                    if processed_audio.contains("alaska") {
                        play_audio(&self.ctx, self.guild_id, "despacito.mp3").await
                    } else if processed_audio.contains("among") {
                        play_audio(&self.ctx, self.guild_id, "sus.mp3").await;
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }
        None
    }
}
