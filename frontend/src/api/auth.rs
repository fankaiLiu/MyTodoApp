use serde::{Deserialize, Serialize};
use crate::api::{ApiClient, ApiResult};
use crate::store::user_store::UserProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub message: String,
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserProfile,
}

pub async fn register(client: &ApiClient, req: &RegisterRequest) -> ApiResult<AuthResponse> {
    client.post("/api/users/register", req).await
}

pub async fn login(client: &ApiClient, req: &LoginRequest) -> ApiResult<AuthResponse> {
    client.post("/api/users/login", req).await
}
