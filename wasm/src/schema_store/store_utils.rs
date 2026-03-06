use std::error::Error;
use serde_json::Value;

pub async fn retrieve_via_http(uri: &str) -> Result<Value, Box<dyn Error + Send + Sync>> {
    #[cfg(target_family = "wasm")]
    {
        use wasm_bindgen::{JsCast, JsValue};
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{window, Response};

        let window = window().unwrap();
        let result: JsValue = JsFuture::from(window.fetch_with_str(uri))
            .await
            .map_err(|e| format!("Fetch failed: {:?}", e))?;

        let resp = result
            .dyn_into::<Response>()
            .map_err(|e| format!("Invalid response type: {:?}", e))?;

        let body = JsFuture::from(
            resp.json()
                .map_err(|e| format!("Failed to get JSON: {:?}", e))?,
        )
        .await
        .map_err(|e| format!("JSON parsing failed: {:?}", e))?;

        let schema: Value = serde_wasm_bindgen::from_value(body)
            .map_err(|e| format!("Deserialization failed: {}", e))?;

        Ok(schema)
    }

    #[cfg(not(target_family = "wasm"))]
    {
        use reqwest::{get, Response};
        let resp: Response = get(uri).await?;
        match resp.json::<Value>().await {
            Ok(schema) => Ok(schema),
            Err(e) => Err(Into::into(e)),
        }
    }
}
