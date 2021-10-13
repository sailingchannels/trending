use std::sync::mpsc::channel;

use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};
use tokio::time::{sleep, Duration};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let six_hours_in_seconds = 6 * 60 * 60 * 1000;

    loop {
        let connection_string = env::var("MONGO_CONNECTION_STRING").unwrap();
        let opts = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(opts)?;

        let channel_repo = ChannelRepository::new(&client);

        let channels = channel_repo.get_all();
        let max_subscribers = channel_repo.get_max_subscribers();

        println!("Wait 6 hours till next execution...");
        sleep(Duration::from_secs(six_hours_in_seconds)).await;
    }
}
