use std::{
    fs::{self, rename},
    path::PathBuf,
};

use rand::{distributions::Alphanumeric, seq::IteratorRandom, thread_rng, Rng};

fn random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn check_if_exists(filename: &str) -> bool {
    let mut dir = fs::read_dir("./responses").expect("among");
    dir.any(|f| f.unwrap().file_name().to_str().unwrap().eq(filename))
}

pub fn random_response() -> PathBuf {
    let mut rng = thread_rng();
    let files = fs::read_dir("./responses").expect("couldnt read dir responses");
    let mut rand_name: String = random_string(10);

    while check_if_exists(&rand_name) {
        rand_name = random_string(10);
    }
    let file = files
        .choose(&mut rng)
        .unwrap()
        .expect("couldnt choose a file 2")
        .path();
    let mut new_file = file.clone();
    new_file.set_file_name(rand_name);
    new_file.set_extension("gif");
    println!(
        "{}, {}",
        file.as_path().to_str().unwrap(),
        new_file.as_path().to_str().unwrap()
    );
    rename(file, &new_file).expect("couldnt rename ze file");
    new_file
}
