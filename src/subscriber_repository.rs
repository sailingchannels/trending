use crate::observation::Observation;
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

    pub async fn get_last_days(
        &self,
        channel_id: &str,
        days: i64,
    ) -> Result<Vec<Observation>, anyhow::Error> {
        let find_options = FindOptions::builder()
            .sort(doc! { "_id.date": -1 })
            .projection(doc! { "subscribers": 1 })
            .limit(days)
            .build();

        let subscribers_cursor = self
            .collection
            .find(doc! { "_id":  channel_id}, find_options)
            .await?;

        let documents: Vec<Document> = subscribers_cursor.try_collect().await?;
        let subscribers = documents
            .iter()
            .map(|document| {
                let subscribers = document.get_i32("subscribers").unwrap();

                Observation {
                    value: f64::from(subscribers),
                    timestamp: document.get_i32("_id.date").unwrap(),
                }
            })
            .collect();

        Ok(subscribers)
    }
}
