use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub error: String,
    pub message: String,
}

impl ApiError {
    pub fn new(status: u16, error: String, message: String) -> Self {
        Self {
            status,
            error,
            message,
        }
    }

    pub fn network(message: String) -> Self {
        Self {
            status: 0,
            error: "Network Error".to_string(),
            message,
        }
    }

    pub fn unauthorized(message: String) -> Self {
        Self {
            status: 401,
            error: "Unauthorized".to_string(),
            message,
        }
    }

    pub fn forbidden(message: String) -> Self {
        Self {
            status: 403,
            error: "Forbidden".to_string(),
            message,
        }
    }

    pub fn not_found(message: String) -> Self {
        Self {
            status: 404,
            error: "Not Found".to_string(),
            message,
        }
    }

    pub fn server_error(message: String) -> Self {
        Self {
            status: 500,
            error: "Server Error".to_string(),
            message,
        }
    }

    pub fn is_unauthorized(&self) -> bool {
        self.status == 401
    }

    pub fn is_forbidden(&self) -> bool {
        self.status == 403
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.status, self.error, self.message)
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
