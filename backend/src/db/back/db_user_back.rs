use crate::models::user::{User, UserStatus};
use crate::models::user_settings::UserSettings;
use crate::utils::utils_passwd;
use anyhow::Result;
use sqlx::{PgPool, Row};

pub struct DbUser;

impl DbUser {
    pub async fn create_user(
        pool: &PgPool,
        username: &str,
        password: &str,
        email: &str,
        phone: &str,
    ) -> Result<User> {
        let user_id = crate::utils::id_generator::generate_user_id();
        let user_reg_time = chrono::Utc::now().timestamp();
        let password_hash = utils_passwd::hash_password(password)?;

        let user_teams: Vec<i64> = vec![];
        let user_teams_json = serde_json::to_value(&user_teams).unwrap();
        let user_settings = UserSettings::default();

        let result = sqlx::query(
            r#"
            INSERT INTO users (user_id, user_username, user_password, user_email, user_reg_time, user_phone, user_teams, user_settings)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING user_id, user_username, user_password, user_email, user_reg_time, user_phone, user_teams, user_last_login_time, user_description, user_avatar, user_status, user_settings
            "#,
        )
        .bind(user_id as i64)
        .bind(username)
        .bind(&password_hash)
        .bind(email)
        .bind(user_reg_time)
        .bind(phone)
        .bind(user_teams_json)
        .bind(serde_json::to_value(&user_settings)?)
        .fetch_one(pool)
        .await?;

        tracing::info!("创建用户成功: user_id = {}", user_id);

        Ok(Self::row_to_user(result)?)
    }

    pub async fn get_user_by_id(pool: &PgPool, user_id: u64) -> Result<Option<User>> {
        let result = sqlx::query(
            r#"
            SELECT user_id, user_username, user_password, user_email, user_reg_time, user_phone, user_teams, user_last_login_time, user_description, user_avatar, user_status, user_settings
            FROM users
            WHERE user_id = $1
            "#,
        )
        .bind(user_id as i64)
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Self::row_to_user(row)?)),
            None => Ok(None),
        }
    }

    pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        let result = sqlx::query(
            r#"
            SELECT user_id, user_username, user_password, user_email, user_reg_time, user_phone, user_teams, user_last_login_time, user_description, user_avatar, user_status, user_settings
            FROM users
            WHERE user_email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Self::row_to_user(row)?)),
            None => Ok(None),
        }
    }

    pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
        let result = sqlx::query(
            r#"
            SELECT user_id, user_username, user_password, user_email, user_reg_time, user_phone, user_teams, user_last_login_time, user_description, user_avatar, user_status, user_settings
            FROM users
            WHERE user_username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Self::row_to_user(row)?)),
            None => Ok(None),
        }
    }

    pub async fn update_user(
        pool: &PgPool,
        user_id: u64,
        username: Option<&str>,
        email: Option<&str>,
        phone: Option<&str>,
        description: Option<&str>,
        avatar: Option<&str>,
    ) -> Result<Option<User>> {
        let mut updates = Vec::new();
        let mut param_count = 1usize;

        if username.is_some() {
            updates.push(format!("user_username = ${}", param_count));
            param_count += 1;
        }
        if email.is_some() {
            updates.push(format!("user_email = ${}", param_count));
            param_count += 1;
        }
        if phone.is_some() {
            updates.push(format!("user_phone = ${}", param_count));
            param_count += 1;
        }
        if description.is_some() {
            updates.push(format!("user_description = ${}", param_count));
            param_count += 1;
        }
        if avatar.is_some() {
            updates.push(format!("user_avatar = ${}", param_count));
            param_count += 1;
        }

        if updates.is_empty() {
            return Self::get_user_by_id(pool, user_id).await;
        }

        let mut query = format!(
            "UPDATE users SET {} WHERE user_id = ${} RETURNING user_id, user_username, user_password, user_email, user_reg_time, user_phone, user_teams, user_last_login_time, user_description, user_avatar, user_status, user_settings",
            updates.join(", "),
            param_count
        );

        let mut row_result = sqlx::query(&query).bind(user_id as i64);

        param_count = 1;
        if let Some(v) = username {
            row_result = row_result.bind(v);
            param_count += 1;
        }
        if let Some(v) = email {
            row_result = row_result.bind(v);
            param_count += 1;
        }
        if let Some(v) = phone {
            row_result = row_result.bind(v);
            param_count += 1;
        }
        if let Some(v) = description {
            row_result = row_result.bind(v);
            param_count += 1;
        }
        if let Some(v) = avatar {
            row_result = row_result.bind(v);
        }

        let result = row_result.fetch_optional(pool).await?;

        match result {
            Some(row) => {
                tracing::info!("更新用户成功: user_id = {}", user_id);
                Ok(Some(Self::row_to_user(row)?))
            }
            None => Ok(None),
        }
    }

    pub async fn delete_user(pool: &PgPool, user_id: u64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM users WHERE user_id = $1")
            .bind(user_id as i64)
            .execute(pool)
            .await?;

        let affected = result.rows_affected();
        tracing::info!("删除用户: user_id = {}, affected = {}", user_id, affected);
        Ok(affected > 0)
    }

    pub async fn update_user_password(
        pool: &PgPool,
        user_id: u64,
        old_password: &str,
        new_password: &str,
    ) -> Result<bool> {
        let user = Self::get_user_by_id(pool, user_id).await?;

        match user {
            Some(u) => {
                let valid = utils_passwd::verify_password(old_password, &u.user_password)?;
                if !valid {
                    return Err(anyhow::anyhow!("旧密码不正确"));
                }

                utils_passwd::validate_password_strength(new_password)?;
                let new_hash = utils_passwd::hash_password(new_password)?;

                let result = sqlx::query("UPDATE users SET user_password = $1 WHERE user_id = $2")
                    .bind(&new_hash)
                    .bind(user_id as i64)
                    .execute(pool)
                    .await?;

                tracing::info!("更新用户密码: user_id = {}", user_id);
                Ok(result.rows_affected() > 0)
            }
            None => Err(anyhow::anyhow!("用户不存在")),
        }
    }

    pub async fn update_last_login_time(pool: &PgPool, user_id: u64) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query("UPDATE users SET user_last_login_time = $1 WHERE user_id = $2")
            .bind(now)
            .bind(user_id as i64)
            .execute(pool)
            .await?;

        tracing::info!("更新用户最后登录时间: user_id = {}", user_id);
        Ok(())
    }

    pub async fn get_user_teams(pool: &PgPool, user_id: u64) -> Result<Vec<u64>> {
        let result = sqlx::query("SELECT user_teams FROM users WHERE user_id = $1")
            .bind(user_id as i64)
            .fetch_optional(pool)
            .await?;

        match result {
            Some(row) => {
                let teams: serde_json::Value = row.get("user_teams");
                let teams: Vec<i64> = serde_json::from_value(teams).unwrap_or_default();
                Ok(teams.into_iter().map(|v| v as u64).collect())
            }
            None => Ok(vec![]),
        }
    }

    pub async fn add_user_team(pool: &PgPool, user_id: u64, team_id: u64) -> Result<bool> {
        let current_teams = Self::get_user_teams(pool, user_id).await?;
        if current_teams.contains(&team_id) {
            return Ok(true);
        }

        let mut new_teams_i64: Vec<i64> = current_teams.into_iter().map(|id| id as i64).collect();
        new_teams_i64.push(team_id as i64);

        let result = sqlx::query("UPDATE users SET user_teams = $1 WHERE user_id = $2")
            .bind(serde_json::to_value(&new_teams_i64)?)
            .bind(user_id as i64)
            .execute(pool)
            .await?;

        tracing::info!(
            "添加用户团队关联: user_id = {}, team_id = {}",
            user_id,
            team_id
        );
        Ok(result.rows_affected() > 0)
    }

    pub async fn remove_user_team(pool: &PgPool, user_id: u64, team_id: u64) -> Result<bool> {
        let current_teams = Self::get_user_teams(pool, user_id).await?;
        if !current_teams.contains(&team_id) {
            return Ok(true);
        }

        // 转换为 i64 以匹配数据库类型
        let new_teams_i64: Vec<i64> = current_teams
            .into_iter()
            .map(|id| id as i64)
            .filter(|&id| id != team_id as i64)
            .collect();

        let result = sqlx::query("UPDATE users SET user_teams = $1 WHERE user_id = $2")
            .bind(serde_json::to_value(&new_teams_i64)?)
            .bind(user_id as i64)
            .execute(pool)
            .await?;

        tracing::info!(
            "移除用户团队关联: user_id = {}, team_id = {}",
            user_id,
            team_id
        );
        Ok(result.rows_affected() > 0)
    }

    fn row_to_user(row: sqlx::postgres::PgRow) -> Result<User> {
        let user_id: i64 = row.get("user_id");
        let user_username: String = row.get("user_username");
        let user_password: String = row.get("user_password");
        let user_email: String = row.get("user_email");
        let user_reg_time: i64 = row.get("user_reg_time");
        let user_phone: String = row.get("user_phone");
        let user_teams: serde_json::Value = row.get("user_teams");
        let user_teams: Vec<i64> = serde_json::from_value(user_teams).unwrap_or_default();
        let user_last_login_time: Option<i64> = row.get("user_last_login_time");
        let user_description: Option<String> = row.get("user_description");
        let user_avatar: Option<String> = row.get("user_avatar");
        let user_status: String = row.get("user_status");
        let user_settings: serde_json::Value = row.get("user_settings");

        let user_status = match user_status.as_str() {
            "Active" => UserStatus::Active,
            "Inactive" => UserStatus::Inactive,
            _ => UserStatus::Active,
        };

        let user_settings: UserSettings = serde_json::from_value(user_settings).unwrap_or_default();

        Ok(User {
            user_id: user_id as u64,
            user_username,
            user_password,
            user_email,
            user_reg_time,
            user_phone,
            user_teams: user_teams.into_iter().map(|v| v as u64).collect(),
            user_last_login_time: user_last_login_time.unwrap_or(0),
            user_description,
            user_settings,
            user_avatar,
            user_status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_crud() {
        let pool = crate::db::pool::create_pool().await.unwrap();

        let user = DbUser::create_user(
            &pool,
            "testuser",
            "TestPass123!",
            "test@example.com",
            "13800138000",
        )
        .await
        .unwrap();

        println!("创建用户: {:?}", user);

        let found = DbUser::get_user_by_id(&pool, user.user_id).await.unwrap();
        println!("查询用户: {:?}", found);

        let updated = DbUser::update_user(
            &pool,
            user.user_id,
            Some("newusername"),
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        println!("更新用户: {:?}", updated);

        let deleted = DbUser::delete_user(&pool, user.user_id).await.unwrap();
        println!("删除用户: {}", deleted);
    }
}
