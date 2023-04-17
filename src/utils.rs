use chrono::{Days, Duration, Months, NaiveDateTime};

pub fn week_start(dt: NaiveDateTime) -> NaiveDateTime {
    dt.checked_sub_days(Days::new(7)).unwrap()
}

pub fn month_start(dt: NaiveDateTime) -> NaiveDateTime {
    dt.checked_sub_months(Months::new(1)).unwrap()
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

    if hours >= 24 {
        return Err("hours greater than 24");
    }

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
    let mut s = String::new();
    let h = d.num_hours();
    if h > 0 {
        s += &format!("{h}:");
    }

    let m = d.num_minutes() % 60;
    s += &format!("{m:0>2}");
    s
}
