use futures::stream::TryStreamExt;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::options::{FindOneOptions, FindOptions};
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
        let find_options = FindOptions::builder()
            .projection(doc! { "subscribers": 1, "lastUploadAt": 1 })
            .build();

        let channels_cursor = self.collection.find(None, find_options).await?;
        let channels: Vec<Document> = channels_cursor.try_collect().await?;

        Ok(channels)
    }

    pub async fn get_max_subscribers(&self) -> Result<f64, anyhow::Error> {
        let find_options = FindOneOptions::builder()
            .sort(doc! { "subscribers": -1 })
            .projection(doc! { "subscribers": 1 })
            .build();

        let channel = self.collection.find_one(None, find_options).await?;
        let subscribers = channel.unwrap().get_i32("subscribers").unwrap();

        Ok(f64::from(subscribers))
    }

    pub async fn get_max_views(&self) -> Result<f64, anyhow::Error> {
        let find_options = FindOneOptions::builder()
            .sort(doc! { "views": -1 })
            .projection(doc! { "views": 1 })
            .build();

        let channel = self.collection.find_one(None, find_options).await?;
        let views = channel.unwrap().get_i32("views").unwrap();

        Ok(f64::from(views))
    }

    pub async fn update_trend(&self, channel_id: &str, trend: f64) {
        let update = doc! {
            "$set": {
                "popularity": {
                    "total": trend,
                    "updatedAt": DateTime::now()
                }
            },
        };

        let update_query = doc! { "_id": channel_id };

        self.collection
            .update_one(update_query, update, None)
            .await
            .unwrap();
    }
}
