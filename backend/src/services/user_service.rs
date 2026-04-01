use crate::db::db_user::DbUser;
use crate::models::user::User;
use crate::utils::jwt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    pub mode: Option<String>,
    pub theme: Option<String>,
}

pub struct UserService;

impl UserService {
    pub async fn register(pool: &PgPool, request: RegisterRequest) -> Result<LoginResponse> {
        if let Some(_) = DbUser::get_user_by_email(pool, &request.email).await? {
            anyhow::bail!("Email already registered");
        }

        if let Some(_) = DbUser::get_user_by_username(pool, &request.username).await? {
            anyhow::bail!("Username already taken");
        }

        crate::utils::validator::validate_user_password(&request.password)?;
        crate::utils::validator::validate_user_email(&request.email)?;
        crate::utils::validator::validate_user_username(&request.username)?;

        let user = DbUser::create_user(
            pool,
            &request.username,
            &request.password,
            &request.email,
            &request.phone,
        )
        .await?;

        let (access_token, refresh_token) =
            jwt::generate_token_pair(user.user_id as i64, user.user_username.clone(), "user".to_string())?;

        DbUser::update_last_login_time(pool, user.user_id).await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            user,
        })
    }

    pub async fn login(pool: &PgPool, request: LoginRequest) -> Result<LoginResponse> {
        let user = DbUser::get_user_by_email(pool, &request.email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        crate::utils::utils_passwd::verify_password(&request.password, &user.user_password)?;

        let (access_token, refresh_token) =
            jwt::generate_token_pair(user.user_id as i64, user.user_username.clone(), "user".to_string())?;

        DbUser::update_last_login_time(pool, user.user_id).await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            user,
        })
    }

    pub async fn get_user_by_id(pool: &PgPool, user_id: u64) -> Result<Option<User>> {
        DbUser::get_user_by_id(pool, user_id).await
    }

    pub async fn update_user(
        pool: &PgPool,
        user_id: u64,
        request: UpdateUserRequest,
    ) -> Result<Option<User>> {
        if let Some(email) = &request.email {
            crate::utils::validator::validate_user_email(email)?;
        }
        if let Some(username) = &request.username {
            crate::utils::validator::validate_user_username(username)?;
        }

        DbUser::update_user(
            pool,
            user_id,
            request.username.as_deref(),
            request.email.as_deref(),
            request.phone.as_deref(),
            request.description.as_deref(),
            request.avatar.as_deref(),
        )
        .await
    }

    pub async fn change_password(
        pool: &PgPool,
        user_id: u64,
        request: ChangePasswordRequest,
    ) -> Result<bool> {
        let user = DbUser::get_user_by_id(pool, user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        crate::utils::utils_passwd::verify_password(&request.old_password, &user.user_password)?;
        crate::utils::validator::validate_user_password(&request.new_password)?;

        DbUser::update_user_password(pool, user_id, &request.new_password).await
    }

    pub async fn update_settings(
        pool: &PgPool,
        user_id: u64,
        request: UpdateSettingsRequest,
    ) -> Result<Option<User>> {
        let user = DbUser::get_user_by_id(pool, user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let mut settings = user.user_settings;

        if let Some(mode) = request.mode {
            match mode.as_str() {
                "Single" => settings.mode = crate::models::user_settings::AppMode::Single,
                "Online" => settings.mode = crate::models::user_settings::AppMode::Online,
                _ => anyhow::bail!("Invalid mode"),
            }
        }

        if let Some(theme) = request.theme {
            match theme.as_str() {
                "Dark" => settings.theme = crate::models::user_settings::Theme::Dark,
                "Light" => settings.theme = crate::models::user_settings::Theme::Light,
                _ => anyhow::bail!("Invalid theme"),
            }
        }

        let settings_json = serde_json::to_string(&settings)?;
        
        let result = sqlx::query("UPDATE users SET user_settings = $1 WHERE user_id = $2")
            .bind(&settings_json)
            .bind(user_id as i64)
            .execute(pool)
            .await?;

        if result.rows_affected() > 0 {
            DbUser::get_user_by_id(pool, user_id).await
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_teams(pool: &PgPool, user_id: u64) -> Result<Vec<i64>> {
        DbUser::get_user_teams(pool, user_id).await
    }
}