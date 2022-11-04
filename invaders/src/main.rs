use crossterm::cursor::Hide;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::{
    cursor::Show,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::error::Error;
use std::io;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    //audio.add(item, &format!("audio/original/{}.wav", item));
    audio.add("explode", "audio/original/man-is-he.mp3");
    audio.add("lose", "audio/original/man-is-he.mp3");
    audio.add("pew", "audio/original/man-is-he.mp3");
    audio.add("move", "audio/original/man-is-he.mp3");
    audio.add("startup", "audio/original/clean-positive-beep.mp3");
    audio.add("run", "audio/original/man-is-he.mp3");

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; //using alternate screen available on terminals
    stdout.execute(Hide)?;

    // Game loop
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {} // for any other key just ignore it
                }
            }
        }
    }

    //cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
