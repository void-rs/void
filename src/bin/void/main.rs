use std::{
    ffi::OsString,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};
use voidmap::{deserialize_screen, init_screen_log, Config};

mod cli;

struct DeleteOnDrop(PathBuf);

impl Drop for DeleteOnDrop {
    fn drop(&mut self) {
        std::fs::remove_file(&self.0).expect("failed to kill lockfile")
    }
}

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
    let mut lock_path = PathBuf::new();
    lock_path.push(&path);
    let mut file_name = lock_path
        .file_name()
        .expect("a filename for the db is needed")
        .to_owned();
    file_name.push(".lock");
    lock_path.set_file_name(file_name);

    let mut lock_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock_path)
        .expect("failed to lock db - is another process using it?");
    write!(
        lock_file,
        "{}::{}",
        hostname::get().unwrap().to_string_lossy(),
        std::process::id()
    )
    .unwrap();
    lock_file.sync_all().unwrap();
    let guard = DeleteOnDrop(lock_path);

    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&path)
        .unwrap();

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
    drop(guard);
}
