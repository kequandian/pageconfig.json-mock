//! MongoDB database connection and operations

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    options::ClientOptions,
    Client, Collection, Database,
};
use serde_json::Value;

/// MongoDB database wrapper
#[derive(Clone)]
pub struct MongoDb {
    db: Database,
}

/// Name of the special collection for storing root-level key-value pairs
const GLOBAL_STORE: &str = "_global_store";

impl MongoDb {
    /// Create a new MongoDB connection
    pub async fn new(uri: &str, db_name: &str) -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);

        // Verify connection
        db.run_command(doc! { "ping": 1 }, None).await?;

        Ok(Self { db })
    }

    /// Get a collection by name
    pub fn collection(&self, name: &str) -> Collection<Document> {
        self.db.collection(name)
    }

    /// Get all documents from a collection
    pub async fn get_all(&self, collection_name: &str) -> Result<Vec<Value>, mongodb::error::Error> {
        let collection = self.collection(collection_name);
        let cursor = collection.find(doc! {}, None).await?;
        let docs: Vec<Document> = cursor.try_collect().await?;

        let values: Vec<Value> = docs
            .into_iter()
            .map(|doc| bson_to_json(Bson::Document(doc)))
            .collect();

        Ok(values)
    }

    /// Get a document by ID from a collection
    pub async fn get_by_id(
        &self,
        collection_name: &str,
        id: i64,
    ) -> Result<Option<Value>, mongodb::error::Error> {
        let collection = self.collection(collection_name);
        let doc = collection.find_one(doc! { "id": id }, None).await?;

        Ok(doc.map(|d| bson_to_json(Bson::Document(d))))
    }

    /// Insert a document into a collection
    pub async fn insert(
        &self,
        collection_name: &str,
        data: Value,
    ) -> Result<Value, mongodb::error::Error> {
        let collection = self.collection(collection_name);
        let doc = json_to_bson_doc(&data);
        collection.insert_one(doc, None).await?;
        Ok(data)
    }

    /// Check if a document with given ID exists
    pub async fn exists_by_id(
        &self,
        collection_name: &str,
        id: i64,
    ) -> Result<bool, mongodb::error::Error> {
        let collection = self.collection(collection_name);
        let count = collection.count_documents(doc! { "id": id }, None).await?;
        Ok(count > 0)
    }

    /// Delete documents from a collection by ID
    pub async fn delete_by_id(
        &self,
        collection_name: &str,
        id: &str,
    ) -> Result<u64, mongodb::error::Error> {
        let collection = self.collection(collection_name);
        // Try both string and integer ID
        let result = if let Ok(id_num) = id.parse::<i64>() {
            collection.delete_many(doc! { "id": id_num }, None).await?
        } else {
            collection.delete_many(doc! { "id": id }, None).await?
        };
        Ok(result.deleted_count)
    }

    /// Delete all documents from a collection
    pub async fn delete_all(&self, collection_name: &str) -> Result<u64, mongodb::error::Error> {
        let collection = self.collection(collection_name);
        let result = collection.delete_many(doc! {}, None).await?;
        Ok(result.deleted_count)
    }

    /// Update a document in a collection
    pub async fn update_by_id(
        &self,
        collection_name: &str,
        id: &str,
        data: Value,
    ) -> Result<bool, mongodb::error::Error> {
        let collection = self.collection(collection_name);
        let doc = json_to_bson_doc(&data);

        let filter = if let Ok(id_num) = id.parse::<i64>() {
            doc! { "id": id_num }
        } else {
            doc! { "id": id }
        };

        let result = collection
            .replace_one(filter, doc, None)
            .await?;

        Ok(result.modified_count > 0)
    }

    // ==================== Global Store Operations ====================

    /// Set a value in the global store (upsert)
    pub async fn set_global(
        &self,
        key: &str,
        value: Value,
    ) -> Result<(), mongodb::error::Error> {
        use mongodb::options::UpdateOptions;
        
        let collection = self.collection(GLOBAL_STORE);
        let bson_value = json_to_bson(&value);

        let options = UpdateOptions::builder().upsert(true).build();
        collection
            .update_one(
                doc! { "_id": key },
                doc! { "$set": { "value": bson_value } },
                options,
            )
            .await?;

        Ok(())
    }

    /// Get a value from the global store
    pub async fn get_global(&self, key: &str) -> Result<Option<Value>, mongodb::error::Error> {
        let collection = self.collection(GLOBAL_STORE);
        let doc = collection.find_one(doc! { "_id": key }, None).await?;

        Ok(doc.and_then(|d| d.get("value").map(|v| bson_to_json(v.clone()))))
    }

    // ==================== Posts Operations (Legacy Compatibility) ====================

    /// Push to posts collection with auto-generated ID
    pub async fn push_post(&self, mut data: Value) -> Result<Value, mongodb::error::Error> {
        let collection = self.collection("posts");

        // Generate ID if not present
        if data.get("id").is_none() {
            let id = chrono_now_millis().to_string();
            data.as_object_mut()
                .map(|obj| obj.insert("id".to_string(), Value::String(id)));
        }

        let doc = json_to_bson_doc(&data);
        collection.insert_one(doc, None).await?;
        Ok(data)
    }

    /// Update a post by ID
    pub async fn update_post(&self, data: Value) -> Result<Option<Value>, mongodb::error::Error> {
        let collection = self.collection("posts");

        let id = data.get("id").and_then(|v| v.as_str());
        if id.is_none() {
            return Ok(None);
        }

        let doc = json_to_bson_doc(&data);
        let result = collection
            .replace_one(doc! { "id": id.unwrap() }, doc, None)
            .await?;

        if result.modified_count > 0 {
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    /// Get a post by ID
    pub async fn get_post(&self, id: &str) -> Result<Option<Value>, mongodb::error::Error> {
        let collection = self.collection("posts");
        let doc = collection.find_one(doc! { "id": id }, None).await?;
        Ok(doc.map(|d| bson_to_json(Bson::Document(d))))
    }

    /// Delete a post by ID
    pub async fn delete_post(&self, id: &str) -> Result<bool, mongodb::error::Error> {
        let collection = self.collection("posts");
        let result = collection.delete_one(doc! { "id": id }, None).await?;
        Ok(result.deleted_count > 0)
    }

    // ==================== Forms Operations ====================

    /// Upsert a form by ID
    pub async fn upsert_form(
        &self,
        id: i64,
        form: Value,
    ) -> Result<(), mongodb::error::Error> {
        use mongodb::options::UpdateOptions;
        
        let collection = self.collection("forms");
        let bson_form = json_to_bson(&form);

        let options = UpdateOptions::builder().upsert(true).build();
        collection
            .update_one(
                doc! { "id": id },
                doc! { "$set": { "id": id, "form": bson_form } },
                options,
            )
            .await?;

        Ok(())
    }

    /// Get all forms or a specific form by ID
    pub async fn get_forms(&self, id: Option<i64>) -> Result<Value, mongodb::error::Error> {
        let collection = self.collection("forms");

        if let Some(form_id) = id {
            let doc = collection.find_one(doc! { "id": form_id }, None).await?;
            if let Some(d) = doc {
                if let Some(form) = d.get("form") {
                    return Ok(bson_to_json(form.clone()));
                }
            }
            Ok(Value::Null)
        } else {
            let cursor = collection.find(doc! {}, None).await?;
            let docs: Vec<Document> = cursor.try_collect().await?;
            let values: Vec<Value> = docs
                .into_iter()
                .map(|doc| bson_to_json(Bson::Document(doc)))
                .collect();
            Ok(Value::Array(values))
        }
    }

    /// Delete a form by ID
    pub async fn delete_form(&self, id: i64) -> Result<bool, mongodb::error::Error> {
        let collection = self.collection("forms");
        let result = collection.delete_one(doc! { "id": id }, None).await?;
        Ok(result.deleted_count > 0)
    }
}

// ==================== Helper Functions ====================

/// Convert JSON Value to BSON
fn json_to_bson(value: &Value) -> Bson {
    match value {
        Value::Null => Bson::Null,
        Value::Bool(b) => Bson::Boolean(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Bson::Int64(i)
            } else if let Some(f) = n.as_f64() {
                Bson::Double(f)
            } else {
                Bson::Null
            }
        }
        Value::String(s) => Bson::String(s.clone()),
        Value::Array(arr) => {
            Bson::Array(arr.iter().map(json_to_bson).collect())
        }
        Value::Object(obj) => {
            let mut doc = Document::new();
            for (k, v) in obj {
                doc.insert(k.clone(), json_to_bson(v));
            }
            Bson::Document(doc)
        }
    }
}

/// Convert JSON Value to BSON Document
fn json_to_bson_doc(value: &Value) -> Document {
    match json_to_bson(value) {
        Bson::Document(doc) => doc,
        _ => Document::new(),
    }
}

/// Convert BSON to JSON Value
fn bson_to_json(bson: Bson) -> Value {
    match bson {
        Bson::Null => Value::Null,
        Bson::Boolean(b) => Value::Bool(b),
        Bson::Int32(i) => Value::Number(i.into()),
        Bson::Int64(i) => Value::Number(i.into()),
        Bson::Double(f) => {
            serde_json::Number::from_f64(f)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
        Bson::String(s) => Value::String(s),
        Bson::Array(arr) => Value::Array(arr.into_iter().map(bson_to_json).collect()),
        Bson::Document(doc) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in doc {
                // Skip MongoDB internal _id field
                if k != "_id" {
                    obj.insert(k, bson_to_json(v));
                }
            }
            Value::Object(obj)
        }
        Bson::ObjectId(oid) => Value::String(oid.to_hex()),
        Bson::DateTime(dt) => Value::String(dt.to_string()),
        _ => Value::Null,
    }
}

/// Get current timestamp in milliseconds
fn chrono_now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_bson_conversion() {
        let json = serde_json::json!({
            "name": "test",
            "count": 42,
            "active": true,
            "items": [1, 2, 3]
        });

        let bson = json_to_bson(&json);
        let back = bson_to_json(bson);

        assert_eq!(json, back);
    }

    #[test]
    fn test_bson_to_json_skips_id() {
        let doc = doc! {
            "_id": "some_id",
            "name": "test"
        };

        let json = bson_to_json(Bson::Document(doc));
        assert!(json.get("_id").is_none());
        assert_eq!(json.get("name").unwrap(), "test");
    }
}
