use futures::stream::TryStreamExt;
use mongodb::bson::Document;
use mongodb::{Client, Collection};

pub struct ChannelRepository {
    collection: Collection<Document>,
}

impl ChannelRepository {
    pub fn new(client: &Client) -> ChannelRepository {
        let db = client.database("sailing-channels");
        let feeds = db.collection::<Document>("channels");

        ChannelRepository { collection: feeds }
    }

    pub async fn get_all(&self) -> Result<Vec<Document>, anyhow::Error> {
        let channels_cursor = self.collection.find(None, None).await?;
        let channels: Vec<Document> = channels_cursor.try_collect().await?;

        Ok(channels)
    }

	pub async fn get_max_subscribers(&self) -> Result<i64, anyhow::Error> {
		let find_options = FindOptions::builder()
			.sort(doc! { "subscribers": -1 })
			.projection(doc! { "subscribers": 1 })
			.limit(1)
			.build();

		let channel: Option<Document> = self.collection.find_one(None, find_options).await?;

        Ok(channel.unwrap().get_i64("subscribers"));
	}

	pub async fn get_max_views(&self) -> Result<i64, anyhow::Error> {
		
	}
}