use crate::models::user_log::{Log_UserLog, UserLogAction};
use anyhow::Result;
use sqlx::{PgPool, Row};

pub struct DbUserLog;

impl DbUserLog {
    /// 创建用户日志
    pub async fn create_user_log(
        pool: &PgPool,
        user_id: u64,
        action: UserLogAction,
        details: Option<&str>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<Log_UserLog> {
        let created_at = chrono::Utc::now().timestamp();
        let action_str = serde_json::to_string(&action)?;

        let result = sqlx::query(
            r#"
            INSERT INTO user_logs (user_id, action, details, ip_address, user_agent, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING log_id, user_id, action, details, ip_address, user_agent, created_at
            "#,
        )
        .bind(user_id as i64)
        .bind(action_str)
        .bind(details)
        .bind(ip_address)
        .bind(user_agent)
        .bind(created_at)
        .fetch_one(pool)
        .await?;

        tracing::info!(
            "创建用户日志成功: user_id = {}, action = {:?}",
            user_id,
            action
        );

        Ok(Self::row_to_user_log(result)?)
    }

    /// 获取用户日志列表（支持多条件筛选和分页）
    pub async fn list_user_logs(
        pool: &PgPool,
        user_id: Option<u64>,
        action: Option<UserLogAction>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Log_UserLog>> {
        let mut query = String::from(
            "SELECT log_id, user_id, action, details, ip_address, user_agent, created_at
             FROM user_logs WHERE 1=1",
        );
        let mut param_count = 1;

        if let Some(uid) = user_id {
            query.push_str(&format!(" AND user_id = ${}", param_count));
            param_count += 1;
        }
        if let Some(ref act) = action {
            let action_str = serde_json::to_string(act)?;
            query.push_str(&format!(" AND action = ${}", param_count));
            param_count += 1;
        }
        if let Some(start) = start_time {
            query.push_str(&format!(" AND created_at >= ${}", param_count));
            param_count += 1;
        }
        if let Some(end) = end_time {
            query.push_str(&format!(" AND created_at <= ${}", param_count));
            param_count += 1;
        }

        query.push_str(" ORDER BY created_at DESC");

        if let Some(lim) = limit {
            query.push_str(&format!(" LIMIT ${}", param_count));
            param_count += 1;
        }
        if let Some(off) = offset {
            query.push_str(&format!(" OFFSET ${}", param_count));
        }

        let mut sql_query = sqlx::query(&query);

        if let Some(uid) = user_id {
            sql_query = sql_query.bind(uid as i64);
        }
        if let Some(ref act) = action {
            let action_str = serde_json::to_string(act)?;
            sql_query = sql_query.bind(action_str);
        }
        if let Some(start) = start_time {
            sql_query = sql_query.bind(start);
        }
        if let Some(end) = end_time {
            sql_query = sql_query.bind(end);
        }
        if let Some(lim) = limit {
            sql_query = sql_query.bind(lim as i64);
        }
        if let Some(off) = offset {
            sql_query = sql_query.bind(off as i64);
        }

        let rows = sql_query.fetch_all(pool).await?;

        let logs: Vec<Log_UserLog> = rows
            .into_iter()
            .map(|row| Self::row_to_user_log(row))
            .collect::<Result<Vec<_>, _>>()?;

        tracing::info!("获取用户日志列表成功: count = {}", logs.len());

        Ok(logs)
    }

    fn row_to_user_log(row: sqlx::postgres::PgRow) -> Result<Log_UserLog> {
        let action_str: String = row.try_get("action")?;
        let action: UserLogAction = serde_json::from_str(&action_str)?;

        Ok(Log_UserLog {
            log_id: row.try_get::<i64, _>("log_id")? as u64,
            user_id: row.try_get::<i64, _>("user_id")? as u64,
            action,
            details: row.try_get("details")?,
            ip_address: row.try_get("ip_address")?,
            user_agent: row.try_get("user_agent")?,
            created_at: row.try_get::<i64, _>("created_at")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_log_crud() {
        let pool = crate::db::pool::create_pool().await.unwrap();

        let test_user = crate::db::db_user::DbUser::create_user(
            &pool,
            "test_user_log",
            "TestPass123!",
            "test_user_log@example.com",
            "1380013800",
        )
        .await
        .unwrap();

        let log = DbUserLog::create_user_log(
            &pool,
            test_user.user_id,
            UserLogAction::Login,
            Some("用户登录成功"),
            Some("127.0.0.1"),
            Some("Mozilla/5.0"),
        )
        .await
        .unwrap();

        assert_eq!(log.user_id, test_user.user_id);
        assert!(matches!(log.action, UserLogAction::Login));
        assert_eq!(log.details, Some("用户登录成功".to_string()));

        let logs = DbUserLog::list_user_logs(
            &pool,
            Some(test_user.user_id),
            Some(UserLogAction::Login),
            None,
            None,
            Some(10),
            None,
        )
        .await
        .unwrap();

        assert!(!logs.is_empty());
        assert_eq!(logs[0].user_id, test_user.user_id);

        let all_logs = DbUserLog::list_user_logs(
            &pool,
            Some(test_user.user_id),
            None,
            None,
            None,
            Some(5),
            None,
        )
        .await
        .unwrap();

        assert!(!all_logs.is_empty());

        crate::db::db_user::DbUser::delete_user(&pool, test_user.user_id)
            .await
            .unwrap();
    }
}
