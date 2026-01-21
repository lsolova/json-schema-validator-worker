use crate::schema_utils::is_http;
use reqwest::{get, Response};
use serde_json::Value;
use std::{collections::HashMap, error::Error, sync::Mutex};

pub struct SchemaStore {
    schema_store: Mutex<HashMap<String, Value>>,
}

impl SchemaStore {
    pub fn new() -> Self {
        SchemaStore {
            schema_store: Mutex::new(HashMap::new()),
        }
    }

    async fn retrieve_via_http(&self, uri: &str) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let resp: Response = get(uri).await?;
        match resp.json::<Value>().await {
            Ok(schema) => Ok(schema),
            Err(e) => Err(Into::into(e)),
        }
    }

    pub async fn add(
        &self,
        uri: &String,
        content: &Value,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let Ok(mut guard) = self.schema_store.lock() else {
            return Err("Schema store is not available (poisoned).".into());
        };
        guard.insert(uri.clone(), content.clone());
        Ok(())
    }

    pub async fn get(&self, uri: &String) -> Result<Option<Value>, Box<dyn Error + Send + Sync>> {
        let Ok(guard) = self.schema_store.lock() else {
            return Err("Schema store is not available (poisoned).".into());
        };
        match guard.get(uri) {
            Some(s) => Ok(Some(s.clone())),
            None => Ok(None),
        }
    }

    pub async fn retrieve(&self, uri: String) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let schema = match self.get(&uri).await {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };

        match schema {
            Some(schema) => Ok(schema),
            None => {
                if is_http(&uri) {
                    match self.retrieve_via_http(&uri).await {
                        Ok(schema) => {
                            let _ = self.add(&uri, &schema).await;
                            Ok(schema)
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    Err(format!("Schema protocol is invalid: '{}'. Only HTTP(S) can be resolved, if it is not in the schema store.", &uri).into())
                }
            }
        }
    }
}
