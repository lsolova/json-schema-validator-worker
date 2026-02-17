use regex::Regex;
use serde_json::Value;
use std::sync::LazyLock;

const HTTPS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^https?://.*").unwrap());
const JSON_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^\\{.*").unwrap());

pub fn is_http(uri: &str) -> bool {
    HTTPS_REGEX.is_match(uri)
}

pub fn is_json(uri: &str) -> bool {
    JSON_REGEX.is_match(uri)
}

pub fn to_json_value(schema: &str) -> Result<Value, String> {
    match serde_json::from_str::<Value>(schema) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("Invalid schema. {}", e).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_http_valid_http_uri() {
        assert!(is_http("http://example.com"));
        assert!(is_http("https://example.com"));
        assert!(is_http("http://example.com/path/to/schema"));
        assert!(is_http("https://example.com/path?query=value"));
    }

    #[test]
    fn test_is_http_invalid_http_uri() {
        assert!(!is_http("id://example.com"));
        assert!(!is_http("example.com"));
        assert!(!is_http("ftp://example.com"));
        assert!(!is_http("{\"type\": \"object\"}"));
        assert!(!is_http(""));
    }

    #[test]
    fn test_is_json_valid_json() {
        assert!(is_json("{\"type\": \"object\"}"));
        assert!(is_json("{}"));
        assert!(is_json("{\"name\": \"test\"}"));
        assert!(is_json("{\"nested\": {\"key\": \"value\"}}"));
    }

    #[test]
    fn test_is_json_invalid_json() {
        assert!(!is_json("http://example.com"));
        assert!(!is_json("id://example.com/schema"));
        assert!(!is_json("example.com"));
        assert!(!is_json("[]"));
        assert!(!is_json(""));
        assert!(!is_json("not json"));
    }

    #[test]
    fn test_is_json_edge_cases() {
        assert!(is_json("{"));
        assert!(!is_json(" {\"type\": \"object\"}"));
    }
}
