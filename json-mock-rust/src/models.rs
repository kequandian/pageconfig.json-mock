//! Response models for the API

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Create a success response with data
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            msg: None,
        }
    }
}

impl ApiResponse<Value> {
    /// Create an error response
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            code: 500,
            data: None,
            msg: Some(msg.into()),
        }
    }

    /// Create an empty success response
    pub fn success_empty() -> Self {
        Self {
            code: 200,
            data: Some(Value::Null),
            msg: None,
        }
    }
}

/// Query parameters for ID-based lookups
#[derive(Debug, Deserialize)]
pub struct IdQuery {
    pub id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let resp = ApiResponse::success(serde_json::json!({"name": "test"}));
        assert_eq!(resp.code, 200);
        assert!(resp.data.is_some());
        assert!(resp.msg.is_none());
    }

    #[test]
    fn test_error_response() {
        let resp = ApiResponse::error("Something went wrong");
        assert_eq!(resp.code, 500);
        assert!(resp.data.is_none());
        assert_eq!(resp.msg.unwrap(), "Something went wrong");
    }
}
