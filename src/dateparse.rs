use time;

use regex::Regex;

pub fn dateparse(input: &str) -> Option<u64> {
    lazy_static! {
        static ref YEARS: Regex = Regex::new(r"(?P<y>\d+)y").unwrap();
        static ref MONTHS: Regex = Regex::new(r"(?P<m>\d+)m").unwrap();
        static ref WEEKS: Regex = Regex::new(r"(?P<w>\d+)w").unwrap();
        static ref DAYS: Regex = Regex::new(r"(?P<d>\d+)d").unwrap();
        static ref HOURS: Regex = Regex::new(r"(?P<h>\d+)h").unwrap();
    }

    let now_in_s = time::get_time().sec as u64;
    let min_in_s = 60;
    let hour_in_s = min_in_s * 60;
    let day_in_s = hour_in_s * 24;
    let week_in_s = day_in_s * 7;
    let month_in_s = day_in_s * 30;
    let year_in_s = day_in_s * 365;

    let input = input.trim();

    if let Some(cap) = YEARS.captures(input) {
        if let Some(years_raw) = cap.get(1) {
            if let Ok(years) = years_raw.as_str().parse::<u64>() {
                return Some(now_in_s - years * year_in_s);
            }
        }
    }

    if let Some(cap) = MONTHS.captures(input) {
        if let Some(months_raw) = cap.get(1) {
            if let Ok(months) = months_raw.as_str().parse::<u64>() {
                return Some(now_in_s - months * month_in_s);
            }
        }
    }

    if let Some(cap) = WEEKS.captures(input) {
        if let Some(weeks_raw) = cap.get(1) {
            if let Ok(weeks) = weeks_raw.as_str().parse::<u64>() {
                return Some(now_in_s - weeks * week_in_s);
            }
        }
    }

    if let Some(cap) = DAYS.captures(input) {
        if let Some(days_raw) = cap.get(1) {
            if let Ok(days) = days_raw.as_str().parse::<u64>() {
                return Some(now_in_s - days * day_in_s);
            }
        }
    }

    if let Some(cap) = HOURS.captures(input) {
        if let Some(hours_raw) = cap.get(1) {
            if let Ok(hours) = hours_raw.as_str().parse::<u64>() {
                return Some(now_in_s - hours * hour_in_s);
            }
        }
    }

    None
}
