extern crate getopts;
extern crate fs2;
extern crate dirs;
extern crate voidmap;

#[macro_use]
extern crate log;

use std::fs::OpenOptions;
use std::io::Read;

use fs2::FileExt;

use voidmap::{Screen, Config, deserialize_screen, init_screen_log};

fn print_usage(program: &str) {
    println!("Usage: {} /path/to/workfile", program);
    std::process::exit(1)
}

fn main() {
    init_screen_log().unwrap();

    let mut args: Vec<String> = std::env::args().collect();
    let program = args.remove(0);
    let default = dirs::home_dir().and_then(|mut h| {
        h.push(".void.db");
        h.to_str().map(|p| p.to_owned())
    });
    let path = args.pop().or(default);

    // load from file if present
    let mut data = vec![];
    let mut f = path.clone()
        .map(|path| {
            OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(path)
                .unwrap_or_else(|e| {
                    print_usage(&*program);
                    panic!("error opening file: {}", e);
                })
        })
        .unwrap();

    // exclusively lock the file
    f.try_lock_exclusive()
        .unwrap_or_else(|_| panic!("another void process is using this path already."));

    f.read_to_end(&mut data).unwrap();

    let saved_screen = deserialize_screen(data).ok();

    let mut screen = saved_screen.unwrap_or_else(Screen::default);
    screen.work_path = path.clone();

    let config = Config::maybe_parsed_from_env().unwrap();
    screen.config = config;

    screen.run();
}
