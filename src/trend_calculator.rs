use crate::observation::Observation;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn calculate(
    historical_subscribers: Vec<Observation>,
    historical_views: Vec<Observation>,
    max_subscribers: f64,
    max_views: f64,
    last_upload_at_timestamp: u64,
) -> f64 {
    const HISTORICAL_VIEW_POPULARITY_FACTOR: f64 = 0.01;
    const HISTORICAL_SUBSCRIBER_POPULARITY_FACTOR: f64 = 0.7;
    const CURRENT_VIEW_POPULARITY_FACTOR: f64 = 0.01;
    const CURRENT_SUBSCRIBER_POPULARITY_FACTOR: f64 = 0.08;
    const LAST_UPLOAD_POPULARITY_FACTOR: f64 = 0.2;

    let historical_subscriber_popularity = calculate_historical_popularity(&historical_subscribers);
    let historical_view_popularity = calculate_historical_popularity(&historical_views);

    let current_subscriber_popularity =
        calculate_current_popularity(&historical_subscribers, max_subscribers);
    let current_view_popularity = calculate_current_popularity(&historical_views, max_views);

    let last_upload_popularity = calculate_last_upload_popularity(last_upload_at_timestamp);

    historical_subscriber_popularity * HISTORICAL_SUBSCRIBER_POPULARITY_FACTOR
        + historical_view_popularity * HISTORICAL_VIEW_POPULARITY_FACTOR
        + current_subscriber_popularity * CURRENT_SUBSCRIBER_POPULARITY_FACTOR
        + current_view_popularity * CURRENT_VIEW_POPULARITY_FACTOR
        + last_upload_popularity * LAST_UPLOAD_POPULARITY_FACTOR
}

fn calculate_last_upload_popularity(last_upload_at_timestamp: u64) -> f64 {
    const ONE_MONTH_IN_SECS: f64 = 2629746.0;

    let max_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64;

    let min_timestamp = max_timestamp - ONE_MONTH_IN_SECS;
    (last_upload_at_timestamp as f64 - min_timestamp) / (max_timestamp - min_timestamp)
}

fn calculate_current_popularity(observations: &Vec<Observation>, max_value: f64) -> f64 {
    let mut obs = observations.to_vec();
    obs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    let latest_value = obs.first().unwrap().value;

    1.0 - latest_value / max_value
}

fn calculate_historical_popularity(observations: &Vec<Observation>) -> f64 {
    let mut obs = observations.to_vec();
    obs.reverse();

    let mut popularity = 0.0;

    let min = obs.iter().fold(f64::INFINITY, |a, b| a.min(b.value));
    let max = obs.iter().fold(-f64::INFINITY, |a, b| a.max(b.value));

    for i in 0..obs.len() - 1 {
        let first_value = &obs[i];
        let second_value = &obs[i + 1];

        let gradient =
            normalize(second_value.value, min, max) - normalize(first_value.value, min, max);

        popularity += gradient;
    }

    popularity
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min).max(1.0)
}
