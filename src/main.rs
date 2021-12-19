use futures::join;
use mongodb::{options::ClientOptions, Client};
use std::env;
use tokio::time::{sleep, Duration};

mod channel_repository;
mod observation;
mod subscriber_repository;
mod timing;
mod trend_calculator;
mod view_repository;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let six_hours_in_seconds = 6 * 60 * 60 * 1000;
    let default_mongodb_conn_string = "mongodb://127.0.0.1:27017".to_string();
    let historical_days: i64 = 5;

    let connection_string =
        env::var("MONGO_CONNECTION_STRING").unwrap_or(default_mongodb_conn_string);

    loop {
        let opts = ClientOptions::parse(&connection_string).await?;
        let client = Client::with_options(opts)?;

        let channel_repo = channel_repository::ChannelRepository::new(&client);
        let view_repo = view_repository::ViewRepository::new(&client);
        let subscriber_repo = subscriber_repository::SubscriberRepository::new(&client);

        let channels_fut = channel_repo.get_all();
        let max_subscribers_fut = channel_repo.get_max_subscribers();
        let max_views_fut = channel_repo.get_max_views();

        let historical_subscribers_fut = subscriber_repo.get_last_days(historical_days);
        let historical_views_fut = view_repo.get_last_days(historical_days);

        let results = join!(
            channels_fut,
            max_subscribers_fut,
            max_views_fut,
            historical_subscribers_fut,
            historical_views_fut
        );

        for channel in channels.iter() {
            let channel_id = channel.get_str("_id").unwrap();
            let channel_last_upload_at = channel.get_i32("lastUploadAt").unwrap_or_default();

            let trend = trend_calculator::calculate(
                historical_subscribers,
                historical_views,
                max_subscribers,
                max_views,
                channel_last_upload_at as u64,
            );

            //channel_repo.update_trend(channel_id, trend).await;
            println!("Update popularity of channel {} to {}", channel_id, trend)
        }

        println!("Wait 6 hours till next execution...");
        sleep(Duration::from_secs(six_hours_in_seconds)).await;
    }
}
