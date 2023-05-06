use std::collections::hash_map::Entry;

use chrono::{Datelike, Days, Duration, NaiveDateTime};
use serde::Deserialize;

use crate::VNDB_NAME_CACHE;

pub fn week_start(dt: NaiveDateTime) -> NaiveDateTime {
    dt.checked_sub_days(Days::new(7)).unwrap()
}

fn parse_hour_min(s: &str) -> Result<Duration, &str> {
    let mut chars = s.chars();
    let mut found_colon = false;

    let mut hours = String::new();
    while let Some(char) = chars.next() {
        if char == ':' {
            found_colon = true;
            break;
        }
        hours += &char.to_string();
    }

    let Ok(hours) = hours.parse::<usize>() else {
        return Err("parsing hour component failed")
    };

    if !found_colon {
        return Err("colon `:` not found");
    }

    let mut minutes = String::new();
    while let Some(char) = chars.next() {
        minutes += &char.to_string();
    }
    let Ok(minutes) = minutes.parse::<usize>() else {
            return Err("parsing minute component failed")
        };

    if minutes >= 60 {
        return Err("minutes greater than 60");
    }

    Ok(Duration::minutes(hours as i64 * 60 + minutes as i64))
}

fn parse_min(s: &str) -> Result<Duration, &str> {
    let mut chars = s.chars();
    let mut minutes = String::new();
    while let Some(char) = chars.next() {
        minutes += &char.to_string();
    }
    let Ok(minutes) = minutes.parse::<usize>() else {
            return Err("parsing minutes failed")
        };

    if minutes >= 60 {
        return Err("minutes greater than 60");
    }

    Ok(Duration::minutes(minutes as i64))
}

pub fn parse_date(s: &str) -> Result<Duration, String> {
    // hour:min format
    let parsed_hour_min = parse_hour_min(s);
    let parsed_hour_min_err = match parsed_hour_min {
        Ok(d) => return Ok(d),
        Err(e) => e,
    };

    // Try `:min` format
    let parsed_min = parse_min(s);
    let parsed_min_err = match parsed_min {
        Ok(d) => return Ok(d),
        Err(e) => e,
    };

    // min format
    Err(format!(
        "Unable to parse duration `{s}`.
Tried `[hour:min]` format, but {parsed_hour_min_err}.
Tried `[min]` format, but {parsed_min_err}."
    ))
}

pub fn fmt_duration(d: chrono::Duration) -> String {
    let h = d.num_hours();
    let m = d.num_minutes() % 60;
    format!("{h}:{m:0>2}")
}

pub fn get_xelieu_tab_name(dt: NaiveDateTime) -> String {
    let month = dt.month();
    let year = dt.year();

    let month = match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    };

    format!("{month}-{year}")
}

pub fn get_vn_name(s: String) -> String {
    #[derive(Deserialize)]
    struct VndbResponse {
        results: Vec<Info>,
    }

    #[derive(Deserialize)]
    struct Info {
        _id: String,
        title: String,
    }

    if !s.starts_with('v') {
        return s;
    }

    let mut cache = VNDB_NAME_CACHE.lock().unwrap();

    match cache.entry(s.clone()) {
        Entry::Vacant(entry) => {
            let resp = ureq::post("https://api.vndb.org/kana/vn").send_json(ureq::json!({
                "filters": ["id", "=", s],
                "fields": "title",
            }));

            let Ok(resp) = resp else { return s };

            let data: VndbResponse = resp.into_json().unwrap();
            let Some(info) = data.results.get(0) else { return s };

            format!(
                "[{}](https://vndb.org/{})",
                entry.insert(info.title.clone()).to_string(),
                s
            )
        }
        Entry::Occupied(o) => format!("[{}](https://vndb.org/{})", o.get().clone(), s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn xelieu_test() {
        assert_eq!(
            get_xelieu_tab_name(
                NaiveDate::from_ymd_opt(2023, 5, 6)
                    .unwrap()
                    .and_hms_opt(9, 10, 11)
                    .unwrap()
            ),
            "May-2023"
        );
    }
}
