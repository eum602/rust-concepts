use crossterm::cursor::Hide;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::{
    cursor::Show,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::frame;
use invaders::frame::new_frame;
use invaders::render::render;
use rusty_audio::Audio;
use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;
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
    stdout.execute(Hide)?; // hide cursor

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let current_frame = match render_rx.recv() {
                Ok(x) => x, // if it is a frame return it
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    // Game loop
    'gameloop: loop {
        // per-frame init
        let curr_frame = new_frame();

        // input
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

        // Draw and render
        let _ = render_tx.send(curr_frame); // silently ignore errors since during startup there won't be child threads to receive the event
        thread::sleep(Duration::from_millis(1)); //make it slower than the render loop
    }

    //cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?; // show cursor
    stdout.execute(LeaveAlternateScreen)?; // leave alternate screen
    terminal::disable_raw_mode()?;
    Ok(())
}
