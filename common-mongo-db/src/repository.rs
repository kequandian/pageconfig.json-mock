use crate::{MongoDb, Result, Error};
use serde_json::Value;
use mongodb::bson::{doc, Bson};
use mongodb::options::UpdateOptions;

/// 通用 JSON 仓储
/// - 支持自定义集合名
/// - 支持自定义数据字段名（form/config）
pub struct JsonRepository {
    db: MongoDb,
    collection_name: String,
    data_field_name: String,  // "form" 或 "config"
}

impl JsonRepository {
    /// 创建新的仓储
    /// - collection_name: 集合名，如 "forms" 或 "pageconfigs"
    /// - data_field_name: 数据字段名，如 "form" 或 "config"
    pub fn new(db: MongoDb, collection_name: impl Into<String>, data_field_name: impl Into<String>) -> Self {
        Self {
            db,
            collection_name: collection_name.into(),
            data_field_name: data_field_name.into(),
        }
    }

    fn json_to_bson(json: &Value) -> Bson {
        match json {
            Value::Null => Bson::Null,
            Value::Bool(b) => Bson::Boolean(*b),
            Value::Number(n) => {
                if n.is_i64() { Bson::Int64(n.as_i64().unwrap()) }
                else if n.is_u64() { Bson::Int64(n.as_u64().unwrap() as i64) }
                else { Bson::Double(n.as_f64().unwrap()) }
            }
            Value::String(s) => Bson::String(s.clone()),
            Value::Array(arr) => Bson::Array(arr.iter().map(|v| Self::json_to_bson(v)).collect()),
            Value::Object(obj) => {
                Bson::Document(obj.iter().map(|(k, v)| (k.clone(), Self::json_to_bson(v))).collect())
            }
        }
    }

    fn bson_to_json(bson: &Bson) -> Result<Value> {
        match bson {
            Bson::Null => Ok(Value::Null),
            Bson::Boolean(b) => Ok(Value::Bool(*b)),
            Bson::Int32(i) => Ok(Value::Number((*i).into())),
            Bson::Int64(i) => Ok(Value::Number((*i).into())),
            Bson::Double(f) => Ok(Value::Number(serde_json::Number::from_f64(*f).unwrap())),
            Bson::String(s) => Ok(Value::String(s.clone())),
            Bson::Array(arr) => Ok(Value::Array(arr.iter().map(|v| Self::bson_to_json(v)).collect::<Result<Vec<Value>>>()?)),
            Bson::Document(doc) => {
                let mut obj = serde_json::Map::new();
                for (k, v) in doc {
                    obj.insert(k.clone(), Self::bson_to_json(v)?);
                }
                Ok(Value::Object(obj))
            }
            _ => Err(Error::InvalidData(format!("Unsupported BSON type: {:?}", bson))),
        }
    }

    /// Upsert JSON 数据
    /// 文档结构: { id: "...", <data_field_name>: {...} }
    pub async fn upsert(&self, id: String, data: Value) -> Result<()> {
        let collection = self.db.collection(&self.collection_name);
        let bson_data = Self::json_to_bson(&data);

        let filter = doc! { "id": &id };
        let update = doc! {
            "$set": {
                "id": &id,
                &self.data_field_name: bson_data
            }
        };
        let options = UpdateOptions::builder().upsert(true).build();

        collection.update_one(filter, update, options).await?;
        Ok(())
    }

    /// 获取 JSON 数据
    pub async fn get(&self, id: &str) -> Result<Option<Value>> {
        let collection = self.db.collection(&self.collection_name);
        let result = collection.find_one(doc! { "id": id }, None).await?;

        Ok(result.and_then(|doc| {
            doc.get(&self.data_field_name).and_then(|v| Self::bson_to_json(v).ok())
        }))
    }

    /// 获取所有数据
    pub async fn get_all(&self) -> Result<Vec<Value>> {
        use futures::TryStreamExt;

        let collection = self.db.collection(&self.collection_name);
        let cursor = collection.find(None, None).await?;

        let docs: Vec<mongodb::bson::Document> = cursor.try_collect().await?;
        let mut results = Vec::new();

        for doc in docs {
            if let Some(data) = doc.get(&self.data_field_name).and_then(|v| Self::bson_to_json(v).ok()) {
                results.push(data);
            }
        }

        Ok(results)
    }
}
