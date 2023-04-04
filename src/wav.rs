use wav::Header;
use wav::header;
use wav::bit_depth::BitDepth;
use std::fs::File;
use std::path::Path;


pub fn to_wav(raw_audio_data : String) -> Result<(), ()>{
    let header = Header::new(header::WAV_FORMAT_IEEE_FLOAT, 1, 44100, 32);
    let mut audio_data :Vec<f32> = vec![];
    /*let audio_data_file = File::open(Path::new("audio")).expect("cannot open audio");
    let mut bufread = BufReader::new(audio_data_file);
    let mut str_data :String = String::new();
    bufread.read_to_string(&mut str_data).unwrap();
    */
    for i in raw_audio_data.split(" ") {
        audio_data.push(i.parse::<f32>().unwrap());
        audio_data.push(0.0);
    }
    
    let bitd :BitDepth = BitDepth::ThirtyTwoFloat(audio_data);
    let mut wav_file = File::create(Path::new("among.wav")).unwrap();
    match wav::write(header, &bitd, &mut wav_file) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
