#![feature(plugin)]
#![plugin(clippy)]
extern crate getopts;
extern crate voidmap;

#[macro_use]
extern crate log;

use std::fs::File;
use std::io::{Error, ErrorKind, Read};

use voidmap::{Screen, deserialize_screen, init_screen_log};

fn print_usage(program: &str) {
    println!("Usage: {} /path/to/workfile", program);
    std::process::exit(1)
}

fn main() {
    init_screen_log().unwrap();

    let mut args: Vec<String> = std::env::args().collect();
    let program = args.remove(0);
    let path = args.pop();

    // load from file if present
    let saved_screen: Option<Screen> = path.clone()
        .and_then(|path| {
            let mut data = vec![];
            match File::open(&path) {
                Err(e) => {
                    println!("error opening file: {}", e);
                    print_usage(&*program);
                }
                Ok(mut f) => {
                    f.read_to_end(&mut data).unwrap();
                }
            }
            Some(data)
        })
        .and_then(|data| deserialize_screen(data).ok());

    let mut screen = saved_screen.unwrap_or_else(Screen::default);
    screen.work_path = path.clone();
    screen.run();
}
