use crate::observation::Observation;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn calculate(
    historical_subscribers: Vec<&Observation>,
    historical_views: Vec<&Observation>,
    last_upload_at_timestamp: u64,
) -> f64 {
    const HISTORICAL_VIEW_POPULARITY_FACTOR: f64 = 0.1;
    const HISTORICAL_SUBSCRIBER_POPULARITY_FACTOR: f64 = 0.6;
    const LAST_UPLOAD_POPULARITY_FACTOR: f64 = 0.3;

    let historical_subscriber_popularity = calculate_historical_popularity(&historical_subscribers);
    let historical_view_popularity = calculate_historical_popularity(&historical_views);

    let last_upload_popularity = calculate_last_upload_popularity(last_upload_at_timestamp);

    historical_subscriber_popularity * HISTORICAL_SUBSCRIBER_POPULARITY_FACTOR
        + historical_view_popularity * HISTORICAL_VIEW_POPULARITY_FACTOR
        + last_upload_popularity * LAST_UPLOAD_POPULARITY_FACTOR
}

fn calculate_last_upload_popularity(last_upload_at_timestamp: u64) -> f64 {
    const ONE_MONTH_IN_SECS: f64 = 2629746.0;

    let max_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64;

    let min_timestamp = max_timestamp - ONE_MONTH_IN_SECS;
    normalize(
        last_upload_at_timestamp as f64,
        min_timestamp,
        max_timestamp,
    )
}

fn calculate_historical_popularity(observations: &Vec<&Observation>) -> f64 {
    if observations.len() == 0 {
        return 0.0;
    }

    let mut obs = observations.to_vec();
    obs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut popularity = 0.;

    let min = obs.iter().fold(f64::INFINITY, |a, b| a.min(b.value));
    let max = obs.iter().fold(-f64::INFINITY, |a, b| a.max(b.value));

    println!("min: {}, max: {}", min, max);

    for i in 1..obs.len() {
        if obs[i - 1].value != 0.0 {
            let rate_of_change = (obs[i].value / obs[i - 1].value) - 1.;

            popularity += rate_of_change;
        }
    }

    popularity / obs.len() as f64
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)).min(1.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn should_normalize_min_value_to_zero() {
        let result = normalize(0.0, 0.0, 11010.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn should_normalize_max_value_to_one() {
        let result = normalize(11010.0, 0.0, 11010.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn should_normalize_center_value_to_one() {
        let result = normalize(5505.0, 0.0, 11010.0);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn should_calculate_last_upload_popularity_to_zero_if_timestamp_older_than_one_month() {
        let result = calculate_last_upload_popularity(15);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn should_calculate_last_upload_popularity_to_one_if_timestamp_in_the_future() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;

        let result = calculate_last_upload_popularity(timestamp * 2);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn should_calculate_last_upload_popularity_within_range_if_few_seconds_ago() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;

        let result = calculate_last_upload_popularity(timestamp - 100);
        assert!(result > 0.0 && result < 1.0);
    }

    #[test]
    fn should_calculate_historical_popularity_of_two_observation_series() {
        let observation_1 = Observation {
            channel_id: "channel a".to_string(),
            value: 10.0,
            timestamp: 1,
        };

        let observation_2 = Observation {
            channel_id: "channel a".to_string(),
            value: 11.0,
            timestamp: 2,
        };

        let observation_3 = Observation {
            channel_id: "channel a".to_string(),
            value: 10.0,
            timestamp: 3,
        };

        let result_a =
            calculate_historical_popularity(&vec![&observation_1, &observation_2, &observation_3]);

        let observation_4 = Observation {
            channel_id: "channel b".to_string(),
            value: 1000.0,
            timestamp: 1,
        };

        let observation_5 = Observation {
            channel_id: "channel b".to_string(),
            value: 1100.0,
            timestamp: 2,
        };

        let observation_6 = Observation {
            channel_id: "channel b".to_string(),
            value: 30.0,
            timestamp: 3,
        };

        let result_b =
            calculate_historical_popularity(&vec![&observation_4, &observation_5, &observation_6]);

        println!("a: {}, b: {}", result_a, result_b);

        assert!(result_a > result_b);
    }
}
