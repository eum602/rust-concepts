use crossterm::cursor::Hide;
use crossterm::{
    cursor::Show,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::error::Error;
use std::io;

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

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; //using alternate screen available on terminals
    stdout.execute(Hide)?;

    //cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
