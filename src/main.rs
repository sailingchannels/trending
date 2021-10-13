use mongodb::{options::ClientOptions, Client};
use std::env;
use tokio::time::{sleep, Duration};

mod channel_repository;
mod trend_calculator;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let six_hours_in_seconds = 6 * 60 * 60 * 1000;
    let default_mongodb_conn_string = "mongodb://127.0.0.1:27017".to_string();

    let connection_string =
        env::var("MONGO_CONNECTION_STRING").unwrap_or(default_mongodb_conn_string);

    loop {
        println!("{}", &connection_string);
        let opts = ClientOptions::parse(&connection_string).await?;
        let client = Client::with_options(opts)?;

        let channel_repo = channel_repository::ChannelRepository::new(&client);

        let channels = channel_repo.get_all().await?;
        let max_subscribers = channel_repo.get_max_subscribers().await?;

        println!("{}", max_subscribers);

        for channel in channels.iter() {
            let trend = trend_calculator::calculate();
        }

        println!("Wait 6 hours till next execution...");
        sleep(Duration::from_secs(six_hours_in_seconds)).await;
    }
}
