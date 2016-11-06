extern crate climate;

use std::io::Read;
use std::fs::File;
use climate::{Screen, deserialize_screen, init_screen_log};

fn main() {
    init_screen_log().unwrap();
    let mut data = vec![];
    let mut f = File::open("/home/t/src/climate/arrowperf.db").unwrap();
    f.read_to_end(&mut data).unwrap();
    let mut screen = deserialize_screen(data).unwrap();
    screen.run();
}
