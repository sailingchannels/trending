use chrono::{DateTime, Duration, Utc};

pub fn get_last_days(days_in_past: i64) -> i64 {
    let start_date: DateTime<Utc> = Utc::now() - Duration::days(days_in_past);

    start_date
        .format("%Y%m%d")
        .to_string()
        .parse::<i64>()
        .unwrap()
}
