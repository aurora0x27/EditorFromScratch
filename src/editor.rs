use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self};
use termion::event::Key;
use termion::input::TermRead;

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    /*
                     * press `Ctrl + q` to exit
                     * */
                    Key::Ctrl('q') => break,
                    _ => println!("{key:?}\r"),
                },
                Err(err) => println!("Error: {err}"),
            }
        }

        disable_raw_mode().unwrap();
    }
}
