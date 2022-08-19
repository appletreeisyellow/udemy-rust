use rusty_audio::Audio;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "explode.wav");
    audio.add("move", "explode.wav");
    audio.add("pew", "explode.wav");
    audio.add("startup", "explode.wav");
    audio.add("win", "explode.wav");

    // Cleanup
    audio.wait();
}
