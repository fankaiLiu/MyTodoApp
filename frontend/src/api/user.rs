use serde::{Deserialize, Serialize};
use crate::api::{ApiClient, ApiResult};
use crate::store::user_store::UserProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTeamsResponse {
    pub teams: Vec<i64>,
}

pub async fn get_user(client: &ApiClient, user_id: u64) -> ApiResult<UserProfile> {
    let path = format!("/api/users/{}", user_id);
    let resp: UserResponse = client.get(&path).await?;
    Ok(resp.user)
}

pub async fn update_user(client: &ApiClient, user_id: u64, req: &UpdateUserRequest) -> ApiResult<UserProfile> {
    let path = format!("/api/users/{}", user_id);
    let resp: UserResponse = client.put(&path, req).await?;
    Ok(resp.user)
}

pub async fn change_password(client: &ApiClient, user_id: u64, req: &ChangePasswordRequest) -> ApiResult<()> {
    let path = format!("/api/users/{}/password", user_id);
    let _: MessageResponse = client.put(&path, req).await?;
    Ok(())
}

pub async fn update_settings(client: &ApiClient, user_id: u64, req: &UpdateSettingsRequest) -> ApiResult<UserProfile> {
    let path = format!("/api/users/{}/settings", user_id);
    let resp: UserResponse = client.put(&path, req).await?;
    Ok(resp.user)
}

pub async fn get_user_teams(client: &ApiClient, user_id: u64) -> ApiResult<Vec<i64>> {
    let path = format!("/api/users/{}/teams", user_id);
    let resp: UserTeamsResponse = client.get(&path).await?;
    Ok(resp.teams)
}

pub async fn get_user_logs(client: &ApiClient, user_id: u64) -> ApiResult<Vec<serde_json::Value>> {
    let path = format!("/api/users/{}/logs", user_id);
    #[derive(Deserialize)]
    struct LogsResponse {
        message: String,
        logs: Vec<serde_json::Value>,
    }
    let resp: LogsResponse = client.get(&path).await?;
    Ok(resp.logs)
}
