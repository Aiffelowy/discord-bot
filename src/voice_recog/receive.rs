use serenity::async_trait;
use songbird::{
    model::payload::{ClientDisconnect, Speaking},
    Event, EventContext, EventHandler as VoiceEventHandler,
};

pub struct Receiver;

impl Receiver {
    pub fn new() -> Self {
        println!("bruh");
        Self {}
    }
}
#[async_trait]
impl VoiceEventHandler for Receiver {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        use EventContext as Ctx;
        match ctx {
            Ctx::SpeakingStateUpdate(Speaking {
                speaking,
                ssrc,
                user_id,
                ..
            }) => {
                println!(
                    "Speaking state update: user {:?}, has ssrc: {:?}, using {:?}",
                    user_id, ssrc, speaking
                );
            }
            Ctx::SpeakingUpdate(data) => {
                println!(
                    "Source {} has {} speaking",
                    data.ssrc,
                    if data.speaking { "started" } else { "stopped" }
                );
            }
            Ctx::VoicePacket(data) => {
                if let Some(audio) = data.audio {
                    println!(
                        "Audio packet first 5 samples: {:?}",
                        audio.get(..5.min(audio.len()))
                    );
                    println!(
                        "Audio packet sequence {:05} has {:04} bytes (before decomp: {}), ssrc: {}",
                        data.packet.sequence.0,
                        audio.len() * std::mem::size_of::<i16>(),
                        data.packet.payload.len(),
                        data.packet.ssrc
                    );
                } else {
                    println!("this packet is not an audio packet");
                }
            }
            Ctx::RtcpPacket(data) => {
                println!("RTCP packet received: {:?}", data.packet);
            }
            Ctx::ClientDisconnect(ClientDisconnect { user_id, .. }) => {
                println!("client disconnected: {}", user_id);
            }
            _ => {
                unimplemented!()
            }
        }
        None
    }
}
