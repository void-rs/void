use time;

use regex::Regex;

pub fn dateparse(input: String) -> Option<u64> {
    lazy_static! {
        static ref DAYS: Regex = Regex::new(r"(?P<d>\d+)d").unwrap();
        static ref HOURS: Regex = Regex::new(r"(?P<h>\d+)h").unwrap();
        static ref MINUTES: Regex = Regex::new(r"(?P<m>\d+)m").unwrap();
    }

    let now = time::get_time().sec as u64;
    let day_in_s = 60 * 60 * 24;
    let hour_in_s = 60 * 60;
    let min_in_s = 60;

    let input = input.trim();

    if let Some(cap) = DAYS.captures(input) {
        if let Some(days_raw) = cap.at(1) {
            if let Ok(days) = days_raw.parse::<u64>() {
                return Some(now - days * day_in_s);
            }
        }
    }

    if let Some(cap) = HOURS.captures(input) {
        if let Some(hours_raw) = cap.at(1) {
            if let Ok(hours) = hours_raw.parse::<u64>() {
                return Some(now - hours * hour_in_s);
            }
        }
    }

    if let Some(cap) = MINUTES.captures(input) {
        if let Some(mins_raw) = cap.at(1) {
            if let Ok(mins) = mins_raw.parse::<u64>() {
                return Some(now - mins * min_in_s);
            }
        }
    }

    None
}
