extern crate getopts;
extern crate voidmap;

#[macro_use]
extern crate log;

use std::fs::OpenOptions;
use std::io::Read;

use voidmap::{Screen, Config, deserialize_screen, init_screen_log};

fn print_usage(program: &str) {
    println!("Usage: {} /path/to/workfile", program);
    std::process::exit(1)
}

fn main() {
    init_screen_log().unwrap();

    let mut args: Vec<String> = std::env::args().collect();
    let program = args.remove(0);
    let default = std::env::home_dir().and_then(|mut h| {
        h.push(".void.db");
        h.to_str().map(|p| p.to_owned())
    });
    let path = args.pop().or(default);

    // load from file if present
    let saved_screen: Option<Screen> = path.clone()
        .and_then(|path| {
            let mut data = vec![];
            let f = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(path);
            match f {
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

    let config = Config::maybe_parsed_from_env().unwrap();
    screen.config = config;

    screen.run();
}
