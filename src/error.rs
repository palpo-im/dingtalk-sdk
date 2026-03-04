//! DingTalk API bindings for the error module.

use std::time::Duration;
use thiserror::Error;

/// Type alias used by the SDK.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
/// Enum model used by this API.
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("API error: {code} - {message}")]
    ApiError { code: i64, message: String },

    #[error("{module} API error: {code} - {message}")]
    ModuleApiError {
        module: &'static str,
        code: i64,
        message: String,
    },

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParam(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HMAC error: {0}")]
    HmacError(String),

    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Rate limit exceeded: {message}")]
    RateLimited {
        message: String,
        retry_after: Option<Duration>,
    },

    #[error("Retry attempts exhausted after {attempts} attempts: {message}")]
    RetryExhausted { attempts: usize, message: String },

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Error {
    /// Executes this helper method.
    pub fn api_error(code: i64, message: impl Into<String>) -> Self {
        Error::ApiError {
            code,
            message: message.into(),
        }
    }

    /// Executes this helper method.
    pub fn auth_error(message: impl Into<String>) -> Self {
        Error::AuthError(message.into())
    }

    /// Executes this helper method.
    pub fn invalid_param(message: impl Into<String>) -> Self {
        Error::InvalidParam(message.into())
    }

    /// Executes this helper method.
    pub fn missing_field(field: impl Into<String>) -> Self {
        Error::MissingField(field.into())
    }

    /// Executes this helper method.
    pub fn module_api_error(module: &'static str, code: i64, message: impl Into<String>) -> Self {
        Error::ModuleApiError {
            module,
            code,
            message: message.into(),
        }
    }

    /// Executes this helper method.
    pub fn rate_limited(message: impl Into<String>, retry_after: Option<Duration>) -> Self {
        Error::RateLimited {
            message: message.into(),
            retry_after,
        }
    }

    /// Executes this helper method.
    pub fn retry_exhausted(attempts: usize, message: impl Into<String>) -> Self {
        Error::RetryExhausted {
            attempts,
            message: message.into(),
        }
    }

    /// Executes this helper method.
    pub fn map_module(self, module: &'static str) -> Self {
        match self {
            Error::ApiError { code, message } => Error::module_api_error(module, code, message),
            other => other,
        }
    }
}
