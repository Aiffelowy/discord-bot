use reqwest::Client;
use std::io::Result;

const IP :&str = "http://192.168.1.148";

pub async fn get_audio_data(prompt: String) -> Result<String> {
    let url = format!("{}:8124/synthesize/{}",IP, prompt);

    let client :Client = Client::new();
    match client.get(url).send().await {
        Ok(response) => return Ok(response.text().await.unwrap()),
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, e)),
    }
}
/*
pub async fn get_riffusion_output(prompt_start: String, prompt_end: String) -> Result<String> {
    let url = format!("{}:3013/run_interface", IP);
    let data = r#"
    {
        "alpha": 0.8,
        "num_inference_steps": 50,
        "seed_image_id": "og_beat",

        "start": {
            "prompt": "prompt"
        }
    }
    "#;
}
*/
