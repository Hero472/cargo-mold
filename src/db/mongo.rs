use mongodb::{Client, Collection, bson::doc};
use serde::{de::DeserializeOwned, Serialize};
use futures::TryStreamExt;

pub struct MongoCollection<T>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    pub collection: Collection<T>,
}

impl<T> MongoCollection<T>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    pub async fn new(uri: &str, db_name: &str, collection_name: &str) -> Self {
        let client = Client::with_uri_str(uri)
            .await
            .expect("Failed to connect to MongoDB");
        let collection = client
            .database(db_name)
            .collection::<T>(collection_name);
        Self { collection }
    }

    pub async fn insert(&self, doc: &T) -> mongodb::error::Result<()> {
        self.collection.insert_one(doc).await?;
        Ok(())
    }

    pub async fn find_one(
        &self,
        filter: mongodb::bson::Document,
    ) -> Option<T> {
        self.collection
            .find_one(filter)
            .await
            .ok()
            .flatten()
    }

    pub async fn find_all(&self) -> mongodb::error::Result<Vec<T>> {
        self.collection
            .find(doc!{})
            .await?
            .try_collect()
            .await
    }

    pub async fn update_one(
        &self,
        filter: mongodb::bson::Document,
        update: mongodb::bson::Document,
    ) -> mongodb::error::Result<bool> {
        let result = self.collection.update_one(filter, update).await?;
        Ok(result.matched_count > 0)
    }

    pub async fn delete_one(
        &self,
        filter: mongodb::bson::Document,
    ) -> mongodb::error::Result<bool> {
        let result = self.collection.delete_one(filter).await?;
        Ok(result.deleted_count > 0)
    }
}

pub struct Db {
    pub client: Client,
    pub db_name: String,
}

impl Db {
    pub async fn connect(uri: &str, db_name: &str) -> Self {
        let client = Client::with_uri_str(uri)
            .await
            .expect("Failed to connect to MongoDB");
        Self { client, db_name: db_name.to_string() }
    }

    /// Get a typed collection — call this once per handler or store in Data<>
    pub fn collection<T>(&self, name: &str) -> MongoCollection<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
    {
        MongoCollection {
            collection: self.client
                .database(&self.db_name)
                .collection::<T>(name),
        }
    }
}