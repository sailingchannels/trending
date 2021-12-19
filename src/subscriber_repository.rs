use crate::{observation::Observation, timing};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mongodb::{Client, Collection};

pub struct SubscriberRepository {
    collection: Collection<Document>,
}

impl SubscriberRepository {
    pub fn new(client: &Client) -> SubscriberRepository {
        let db = client.database("sailing-channels");
        let feeds = db.collection::<Document>("subscribers");

        SubscriberRepository { collection: feeds }
    }

    pub async fn get_last_days(&self, days: i64) -> Result<Vec<Observation>, anyhow::Error> {
        let find_options = FindOptions::builder()
            .projection(doc! { "subscribers": 1, "_id.date": 1, "_id.channel": 1 })
            .build();

        let start_date = timing::get_last_days(days);

        let subscribers_cursor = self
            .collection
            .find(doc! { "_id.date":  {"$gte": start_date}}, find_options)
            .await?;

        let documents: Vec<Document> = subscribers_cursor.try_collect().await?;
        let subscribers = documents
            .iter()
            .map(|document| {
                let subscribers = document.get_i32("subscribers").unwrap();
                let id = document.get_document("_id").unwrap();

                Observation {
                    channel_id: id.get_str("channel").unwrap().to_string(),
                    value: f64::from(subscribers),
                    timestamp: id.get_i32("date").unwrap_or(-1),
                }
            })
            .filter(|observation| observation.timestamp > 0)
            .collect();

        Ok(subscribers)
    }
}
