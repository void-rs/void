use clap::{App, Arg};

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

pub fn create<'a>() -> App<'a, 'a> {
    App::new(APP_NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .arg(Arg::with_name("PATH").takes_value(true).required(false))
}
