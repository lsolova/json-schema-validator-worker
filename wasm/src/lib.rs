use schema_retriever::SchemaRetriever;
use schema_store::SchemaStore;
use serde_json::Value;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

mod schema_retriever;
mod schema_store;

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
        if !id.starts_with("id://") {
            return Err("Schema ID is invalid. It should start with id:// protocol.".into());
        };
        let Ok(schema_content) = serde_json::from_str::<Value>(schema) else {
            return Err("Invalid schema".into())
        };
        let Ok(saved) = self.schema_store.add(&id, &schema_content).await else {
            return Err("Schema saving failed".into())
        };
        Ok(saved)
    }

    pub async fn validate(&self, schema: &str, content: &str) -> Result<bool, String> {
        let json_schema = match serde_json::from_str::<Value>(schema) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Invalid schema {}", e).into());
            }
        };

        let json_content = match serde_json::from_str::<Value>(content) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("Invalid content {}", e).into());
            }
        };

        let validator_result = jsonschema::async_options()
            .with_retriever(SchemaRetriever::new(Arc::clone(&self.schema_store)))
            .build(&json_schema)
            .await;

        let validator = match validator_result {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Schema compilation failed {}", e).into());
            }
        };

        match validator.validate(&json_content) {
            Ok(_) => Ok(true),
            Err(error) => {
                println!("Validation error: {}", error);
                Err(error.to_string())
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
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn it_fails() {
        let schema = json!({"type": "string"});
        let content = json!(42);
        let schema_validator = WasmSchemaValidator::new();
        let result = schema_validator
            .validate(&schema.to_string(), &content.to_string())
            .await;
        assert!(result.is_err());
    }
}
