use mongodb::{Client, ClientSession, Collection, bson::{Bson, Document, doc}};
use serde::{de::DeserializeOwned, Serialize};
use futures::TryStreamExt;
use crate::common::{errors::AppError, extractors::pagination::{PaginatedResponse, PaginationQuery, SortDirection}};

pub trait MongoPaginationExt {
    fn to_filter(&self, searchable_fields: &[&str]) -> Document;
    fn to_sort(&self) -> Document;
}

impl MongoPaginationExt for PaginationQuery {
    fn to_filter(&self, searchable_fields: &[&str]) -> Document {
        let mut filter = doc! {};
        
        // Text Search Logic
        if let Some(ref query) = self.q {
            if !query.is_empty() {
                let regex = doc! { "$regex": query, "$options": "i" };
                let or: Vec<Document> = searchable_fields.iter()
                    .map(|&f| doc! { f: regex.clone() })
                    .collect();
                if !or.is_empty() { filter.insert("$or", or); }
            }
        }

        // Dynamic Filters Logic
        for (key, value) in &self.filters {
            let bson_val = match value.to_lowercase().as_str() {
                "true" => Bson::Boolean(true),
                "false" => Bson::Boolean(false),
                _ => {
                    // Try to parse as Int/Float, fallback to String
                    if let Ok(i) = value.parse::<i64>() { Bson::Int64(i) }
                    else if let Ok(f) = value.parse::<f64>() { Bson::Double(f) }
                    else { Bson::String(value.to_string()) }
                }
            };
            filter.insert(key, bson_val);
        }
        filter
    }

    fn to_sort(&self) -> Document {
        let field = self.sort_by.as_deref().unwrap_or("_id");
        let order = self.sort_order.as_ref().unwrap_or(&SortDirection::Desc).to_int();
        doc! { field: order }
    }
}

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

    pub async fn insert(
        &self, 
        doc: &T, 
        session: Option<&mut ClientSession>
    ) -> Result<(), AppError> {
        let mut action = self.collection.insert_one(doc);
        
        // If a session is provided, attach it to the action
        if let Some(s) = session { action = action.session(s); }

        action.await.map_err(AppError::Database)?;
        Ok(())
    }

    pub fn parse_id(&self, id: &str) -> Result<mongodb::bson::oid::ObjectId, AppError> {
        mongodb::bson::oid::ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid ID format".into()))
    }

    pub async fn find_by_id(
        &self, 
        id: &str, 
        session: Option<&mut ClientSession>
    ) -> Result<T, AppError> {
        let obj_id = self.parse_id(id)?;
        let filter = doc! { "_id": obj_id };
        
        let mut action = self.collection.find_one(filter);
        if let Some(s) = session { action = action.session(s); }

        action.await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound(format!("Entity {} not found", id)))
    }

    pub async fn find_one(&self, filter: Document) -> Result<T, AppError> {
        self.collection
            .find_one(filter)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound("Resource not found".into()))
    }

    pub async fn find_all(&self) -> Result<Vec<T>, AppError> {
        self.collection
            .find(doc! {})
            .await
            .map_err(AppError::Database)?
            .try_collect()
            .await
            .map_err(AppError::Database)
    }

    pub async fn find_many(&self, filter: Document) -> Result<Vec<T>, AppError> {
        self.collection
            .find(filter)
            .await
            .map_err(AppError::Database)?
            .try_collect()
            .await
            .map_err(AppError::Database)
    }

    pub async fn aggregate(&self, pipeline: Vec<Document>) -> Result<Vec<Document>, AppError> {
        self.collection
            .aggregate(pipeline)
            .await
            .map_err(AppError::Database)?
            .try_collect()
            .await
            .map_err(AppError::Database)
    }

    pub async fn update_one(
        &self, 
        filter: Document, 
        update: Document, 
        session: Option<&mut ClientSession>
    ) -> Result<bool, AppError> {
        let mut action = self.collection.update_one(filter, update);
        if let Some(s) = session { action = action.session(s); }

        let result = action.await.map_err(AppError::Database)?;
        Ok(result.matched_count > 0)
    }

    pub async fn update_many(&self, filter: Document, update: Document) -> Result<u64, AppError> {
        let result = self.collection
            .update_many(filter, update)
            .await
            .map_err(AppError::Database)?;
        Ok(result.modified_count)
    }

    pub async fn delete_one(
        &self, 
        filter: Document, 
        session: Option<&mut ClientSession>
    ) -> Result<bool, AppError> {
        let mut action = self.collection.delete_one(filter);
        if let Some(s) = session { action = action.session(s); }

        let result = action.await.map_err(AppError::Database)?;
        Ok(result.deleted_count > 0)
    }

    pub async fn find_paginated(
        &self,
        query: PaginationQuery,
        searchable_fields: Vec<&str>,
    ) -> Result<PaginatedResponse<T>, AppError> {
        let filter = query.to_filter(&searchable_fields);
        let sort = query.to_sort();

        let total = self.collection
            .count_documents(filter.clone())
            .await
            .map_err(AppError::Database)?;

        let mut cursor = self.collection
            .find(filter)
            .limit(query.get_limit())
            .skip(query.skip())
            .sort(sort)
            .await
            .map_err(AppError::Database)?;

        let mut items = Vec::new();
        while let Ok(Some(item)) = cursor.try_next().await {
            items.push(item);
        }

        Ok(PaginatedResponse::new(items, total, &query))
    }

    pub fn raw(&self) -> &Collection<T> {
        &self.collection
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

    pub async fn start_transaction(&self) -> Result<mongodb::ClientSession, AppError> {
        let mut session = self.client
            .start_session()
            .await
            .map_err(AppError::Database)?;
            
        session.start_transaction()
            .await
            .map_err(AppError::Database)?;
            
        Ok(session)
    }

    pub async fn commit_transaction(&self, mut session: mongodb::ClientSession) -> Result<(), AppError> {
        session.commit_transaction()
            .await
            .map_err(AppError::Database)
    }

    pub async fn abort_transaction(&self, mut session: mongodb::ClientSession) -> Result<(), AppError> {
        session.abort_transaction()
            .await
            .map_err(AppError::Database)
    }
}