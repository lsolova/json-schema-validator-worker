use crate::schema_retriever::SchemaRetriever;
use crate::schema_store::SchemaStore;
use crate::schema_utils::{is_id, is_json, is_uri, to_json_value};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

mod schema_retriever;
mod schema_store;
mod schema_utils;

#[wasm_bindgen]
pub struct WasmSchemaValidator {
    schema_store: Arc<SchemaStore>,
}

#[wasm_bindgen]
impl WasmSchemaValidator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmSchemaValidator {
        WasmSchemaValidator {
            schema_store: Arc::new(SchemaStore::new()),
        }
    }

    pub async fn add_schema(&self, id: String, schema: &str) -> Result<(), String> {
        if !is_id(&id) {
            return Err("Schema ID is invalid. It should start with id:// protocol.".into());
        };
        let schema_content = to_json_value(schema)?;
        let Ok(saved) = self.schema_store.add(&id, &schema_content).await else {
            return Err("Schema saving failed".into());
        };
        Ok(saved)
    }

    pub async fn validate(&self, schema: &str, content: &str) -> Result<(), String> {
        let json_schema = match schema {
            s if is_uri(s) => match self.schema_store.retrieve(schema.to_string()).await {
                Ok(s) => s,
                Err(e) => {
                    return Err(format!("Schema not found. {}", e).into());
                }
            },
            s if is_json(s) => to_json_value(schema)?,
            _ => {
                return Err("Schema must be a valid URI or JSON string".into());
            }
        };

        let json_content = match serde_json::from_str::<Value>(content) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("Invalid content {}", e).into());
            }
        };

        let async_validator = jsonschema::async_options()
            .with_retriever(SchemaRetriever::new(Arc::clone(&self.schema_store)))
            .build(&json_schema)
            .await;

        let validator = match async_validator {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Schema compilation failed {}", e).into());
            }
        };

        let evaluation = validator.evaluate(&json_content);
        match evaluation.flag().valid {
            true => Ok(()),
            false => {
                let mut error_map = HashMap::<String, String>::new();
                evaluation
                    .iter_errors()
                    .for_each(|f| { error_map.insert(f.instance_location.to_string(), f.error.to_string()); });
                let err_value = serde_json::to_string(&error_map).unwrap_or_else(|_| "Failed to serialize errors".to_string());
                Err(err_value.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn it_works() {
        let schema = json!({"type": "string"});
        let content = json!("Hello world");
        let schema_validator = WasmSchemaValidator::new();
        let result = schema_validator
            .validate(&schema.to_string(), &content.to_string())
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn it_fails() {
        let schema = json!({"type": "object", "properties": {"name": {"type": "string"}, "initials": { "type": "string"}}, "required": ["initials", "name"]});
        let content = json!({ "name": 123 });
        let schema_validator = WasmSchemaValidator::new();
        let result = schema_validator
            .validate(&schema.to_string(), &content.to_string())
            .await;
        assert!(result.is_err());
        print!("{}", result.unwrap_err());
    }
}
