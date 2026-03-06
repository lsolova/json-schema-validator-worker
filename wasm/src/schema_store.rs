use crate::{schema_store::store_utils::retrieve_via_http, schema_utils::is_http};
use serde_json::Value;
use std::{collections::HashMap, error::Error, io:: {Error as IoError, ErrorKind}, sync::Mutex};

mod store_utils;

pub struct SchemaStore {
    schema_store: Mutex<HashMap<String, Value>>,
}

impl SchemaStore {
    pub fn new() -> Self {
        SchemaStore {
            schema_store: Mutex::new(HashMap::new()),
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
                    match retrieve_via_http(&uri).await {
                        Ok(schema) => {
                            let _ = self.add(&uri, &schema).await;
                            Ok(schema)
                        }
                        Err(e) => Err(Box::new(
                            IoError::new(
                                ErrorKind::Other,
                                format!("Failed to retrieve schema from '{}': {}", &uri, e)
                            )
                        ))
                    }
                } else {
                    Err(format!("Schema protocol is invalid: '{}'. Only HTTP(S) can be resolved, if it is not in the schema store.", &uri).into())
                }
            }
        }
    }
}
