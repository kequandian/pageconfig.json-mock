use mongodb::{Client, Database, options::ClientOptions};
use anyhow::Result;

pub struct MongoDb {
    pub db: Database,
}

impl MongoDb {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);
        Ok(MongoDb { db })
    }

    pub fn collection(&self, name: &str) -> mongodb::Collection<mongodb::bson::Document> {
        self.db.collection(name)
    }
}
