use crate::schema_store::SchemaStore;
use jsonschema::{AsyncRetrieve, Uri};
use serde_json::Value;
use std::sync::Arc;

pub struct SchemaRetriever {
    schema_store: Arc<SchemaStore>,
}

impl SchemaRetriever {
    pub fn new(schema_store: Arc<SchemaStore>) -> Self {
        SchemaRetriever { schema_store }
    }
}

#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
impl AsyncRetrieve for SchemaRetriever {
    async fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let url = uri.as_str();

        if url.starts_with("http://") || url.starts_with("https://") {
            match self.schema_store.retrieve(url.to_string()).await {
                Ok(schema) => Ok(schema),
                Err(e) => Err(Into::into(e)),
            }
        } else {
            Err("Schema URI's protocol not supported. Check the passed URI.".into())
        }
    }
}
