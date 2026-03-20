use serde_json::Value;
use std::error::Error;

async fn retrieve_via_http_reqwest(uri: &str) -> Result<Value, Box<dyn Error + Send + Sync>> {
    use reqwest::{get, Response};

    let resp: Response = get(uri).await?;
    match resp.json::<Value>().await {
        Ok(schema) => Ok(schema),
        Err(e) => Err(Into::into(e)),
    }
}

pub async fn retrieve_via_http(uri: &str) -> Result<Value, Box<dyn Error + Send + Sync>> {
    #[cfg(target_family = "wasm")]
    {
        use wasm_bindgen::{JsCast, JsValue};
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{console, window, Response};

        let window = match window() {
            Some(w) => w,
            None => {
                console::warn_1(&JsValue::from_str(
                    format!(
                        "Window is undefined. Fallback to custom fetch when retrieve '{}'",
                        &uri
                    )
                    .as_str(),
                ));
                return retrieve_via_http_reqwest(uri).await;
            }
        };

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
        return retrieve_via_http_reqwest(uri)?;
    }
}
