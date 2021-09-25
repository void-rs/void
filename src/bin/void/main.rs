use fs2::FileExt;
use std::{ffi::OsString, fs::OpenOptions, io::Read};
use voidmap::{deserialize_screen, init_screen_log, Config};

mod cli;

fn main() {
    // Initialise the CLI parser
    let app = cli::create();
    let matches = app.get_matches();

    // Initialise screen logger
    init_screen_log().unwrap();

    let path: OsString = matches
        .value_of("PATH")
        .map(OsString::from)
        .or_else(|| {
            dirs::home_dir().map(|mut h| {
                h.push(".void.db");
                h.into_os_string()
            })
        })
        .unwrap();

    // load from file if present
    let mut data = vec![];
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&path)
        .unwrap();

    // exclusively lock the file
    f.try_lock_exclusive()
        .unwrap_or_else(|_| panic!("Another `void` process is using this path already!"));

    f.read_to_end(&mut data).unwrap();
    let saved_screen = deserialize_screen(data).expect("invalid screen");

    // Initialise the main working screen
    let mut screen = saved_screen/*.unwrap_or_else(Screen::default)*/;

    screen.work_path = matches
        .value_of("PATH")
        .map(|s| s.into())
        .or_else(|| Some(path.into_string().unwrap()));

    if let Some(autosave_every) = matches
        .value_of("AUTOSAVE_EVERY")
        .and_then(|s| s.parse().ok())
    {
        screen.autosave_every = autosave_every;
    }

    let config = Config::maybe_parsed_from_env().unwrap();
    screen.config = config;

    screen.run();
}
