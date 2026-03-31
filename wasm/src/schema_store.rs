use crate::{schema_store::store_utils::retrieve_via_http, schema_utils::is_http};
use jsonschema::Validator;
use serde_json::Value;
use std::{collections::HashMap, error::Error, sync::Mutex};

mod store_utils;

pub struct SchemaStore {
    schema_store: Mutex<HashMap<String, Value>>,
    validator_store: Mutex<HashMap<String, Validator>>,
}

impl SchemaStore {
    pub fn new() -> Self {
        SchemaStore {
            schema_store: Mutex::new(HashMap::new()),
            validator_store: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_schema(
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

    pub async fn add_validator(&self, uri: &String, validator: &Validator) -> Result<(), Box<dyn Error + Send + Sync>> {
        let Ok(mut guard) = self.validator_store.lock() else {
            return Err("Validator store is not available (poisoned)".into());
        };
        guard.insert(uri.clone(), validator.clone());
        Ok(())
    }

    pub async fn get_schema(&self, uri: &String) -> Result<Option<Value>, Box<dyn Error + Send + Sync>> {
        let Ok(guard) = self.schema_store.lock() else {
            return Err("Schema store is not available (poisoned).".into());
        };
        match guard.get(uri) {
            Some(s) => Ok(Some(s.clone())),
            None => Ok(None),
        }
    }

    pub async fn get_validator(&self, uri: &String) -> Result<Option<Validator>, Box<dyn Error + Send + Sync>> {
        let Ok(guard) = self.validator_store.lock() else {
            return Err("Validator store is not available (poisoned).".into());
        };
        match guard.get(uri) {
            Some(s) => Ok(Some(s.clone())),
            None => Ok(None),
        }
    }

    pub async fn retrieve(&self, uri: String) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let schema = match self.get_schema(&uri).await {
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
                            let _ = self.add_schema(&uri, &schema).await;
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
