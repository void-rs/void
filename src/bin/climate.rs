#![feature(plugin)]
#![plugin(clippy)]
extern crate getopts;
extern crate climate;

#[macro_use]
extern crate log;

use std::fs::File;
use std::io::{Error, ErrorKind, Read};

use climate::{Screen, deserialize_screen, init_screen_log};
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [--file=/path/to/workfile]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_workfile_path() -> std::io::Result<String> {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "file", "use a workfile", "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    matches.opt_str("f").ok_or_else(|| Error::new(ErrorKind::Other, "no path provided"))
}

fn main() {
    init_screen_log().unwrap();

    // load from file if present
    let saved_screen: Option<Screen> = parse_workfile_path()
        .and_then(|path| {
            let mut data = vec![];
            let mut f = try!(File::open(path));
            f.read_to_end(&mut data).unwrap();
            Ok(data)
        })
        .ok()
        .and_then(|data| deserialize_screen(data).ok());

    let mut screen = saved_screen.unwrap_or_else(Screen::default);
    screen.work_path = parse_workfile_path().ok();
    screen.run();
}
