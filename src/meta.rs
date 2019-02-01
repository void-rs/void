use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    num,
};

use hyper::{self, client::Client};
use time;

lazy_static! {
    static ref LOC: (f32, f32) = gps_query().unwrap_or_else(|e| {
        error!("failed to get gps: {:?}", e);
        (0.0, 0.0)
    });
}

#[derive(Debug, Clone)]
pub struct Meta {
    pub ctime: u64,
    pub mtime: u64,
    pub finish_time: Option<u64>,
    pub due: Option<u64>,
    pub gps: (f32, f32),
    pub tags: HashMap<String, String>,
}

impl Default for Meta {
    fn default() -> Meta {
        let now = time::get_time().sec as u64;
        Meta {
            ctime: now,
            mtime: now,
            finish_time: None,
            due: None,
            gps: *LOC,
            tags: HashMap::new(),
        }
    }
}

impl Meta {
    pub fn bump_mtime(&mut self) { self.mtime = time::get_time().sec as u64; }

    pub fn finish(&mut self) { self.finish_time = Some(time::get_time().sec as u64); }

    pub fn unfinish(&mut self) { self.finish_time = None; }

    pub fn at(&self) -> u64 { self.finish_time.unwrap_or(self.mtime) }
}

fn gps_query() -> Result<(f32, f32), GpsError> {
    if env::var("LOCATION_QUERY").is_err() {
        info!("GPS lookup disabled. Enable by setting LOCATION_QUERY env var.");
        return Err(GpsError::Other("GPS lookup disabled".to_owned()));
    }
    let client = Client::new();
    let mut res = client.get("http://ipinfo.io/loc").send()?;
    let mut text_res = String::new();
    res.read_to_string(&mut text_res)?;
    let parts = text_res.trim().split(',').collect::<Vec<_>>();

    if parts.len() == 2 {
        let lat = parts[0].parse::<f32>()?;
        let lon = parts[1].parse::<f32>()?;
        Ok((lat, lon))
    } else {
        let err_string = format!("unable to parse response: {:?}", text_res);
        let error = GpsError::Other(err_string);
        Err(error)
    }
}

#[derive(Debug)]
enum GpsError {
    Hyper(hyper::Error),
    Io(io::Error),
    Parse(num::ParseFloatError),
    Other(String),
}

impl From<hyper::Error> for GpsError {
    fn from(err: hyper::Error) -> GpsError { GpsError::Hyper(err) }
}

impl From<io::Error> for GpsError {
    fn from(err: io::Error) -> GpsError { GpsError::Io(err) }
}

impl From<num::ParseFloatError> for GpsError {
    fn from(err: num::ParseFloatError) -> GpsError { GpsError::Parse(err) }
}
