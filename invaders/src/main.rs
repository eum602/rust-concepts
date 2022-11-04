use std::error::Error;
use rusty_audio::Audio;
fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    //audio.add(item, &format!("audio/original/{}.wav", item));
    audio.add("explode", "audio/original/man-is-he.mp3");
    audio.add("lose", "audio/original/man-is-he.mp3");
    audio.add("pew", "audio/original/man-is-he.mp3");
    audio.add("move", "audio/original/man-is-he.mp3");
    audio.add("startup", "audio/original/man-is-he.mp3");
    audio.add("run", "audio/original/man-is-he.mp3");

    audio.play("pew");

    //cleanup
    audio.wait();
    Ok(())
}
