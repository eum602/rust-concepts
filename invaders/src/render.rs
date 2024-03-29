use std::io::{Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;

use crate::frame::Frame;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        // if force is true, clear the screen
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap(); // clear the whole screen
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap(); // set the background color to black
    }

    for (x, col) in current_frame.iter().enumerate() {
        // checking by columns first
        for (y, s) in col.iter().enumerate() {
            // then by rows
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}
