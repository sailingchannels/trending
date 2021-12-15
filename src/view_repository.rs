use crate::observation::Observation;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mongodb::{Client, Collection};

pub struct ViewRepository {
    collection: Collection<Document>,
}

impl ViewRepository {
    pub fn new(client: &Client) -> ViewRepository {
        let db = client.database("sailing-channels");
        let col = db.collection::<Document>("views");

        ViewRepository { collection: col }
    }

    pub async fn get_last_days(
        &self,
        channel_id: &str,
        days: i64,
    ) -> Result<Vec<Observation>, anyhow::Error> {
        let find_options = FindOptions::builder()
            .sort(doc! { "_id.date": -1 })
            .projection(doc! { "views": 1, "_id.date": 1 })
            .limit(days)
            .build();

        let views_cursor = self
            .collection
            .find(doc! { "_id.channel":  channel_id}, find_options)
            .await?;

        let documents: Vec<Document> = views_cursor.try_collect().await?;
        let views = documents
            .iter()
            .map(|document| {
                let views = document.get_i32("views").unwrap();
                let id = document.get_document("_id").unwrap();

                Observation {
                    value: f64::from(views),
                    timestamp: id.get_i32("date").unwrap_or(-1),
                }
            })
            .filter(|observation| observation.timestamp > 0)
            .collect();

        Ok(views)
    }
}
