use std::collections::HashSet;

use crate::models::task::{Task, TaskStatus};
use anyhow::Result;
use sqlx::{PgPool, Row};

pub struct DbTask;

#[derive(Debug, Clone, Default)]
pub struct TaskFilter {
    pub leader_id: Option<u64>,
    pub team_id: Option<u64>,
    pub status: Option<TaskStatus>,
    pub priority: Option<u8>,
    pub deadline_before: Option<i64>,
    pub deadline_after: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl DbTask {
    pub async fn create_task(
        pool: &PgPool,
        name: &str,
        leader_id: u64,
        description: Option<&str>,
        keywords: Option<Vec<String>>,
        priority: u8,
        deadline: Option<i64>,
        team_id: Option<u64>,
    ) -> Result<Task> {
        let task_id = crate::utils::id_generator::generate_task_id();
        let task_create_time = chrono::Utc::now().timestamp();
        let keywords = keywords.unwrap_or_default();

        let result = sqlx::query(
            r#"
            INSERT INTO tasks (task_id, task_name, task_description, task_keywords, task_priority, task_deadline, task_status, task_create_time, task_leader_id, task_team_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING task_id, task_name, task_description, task_keywords, task_priority, task_deadline, task_complete_time, task_status, task_create_time, task_leader_id, task_team_id, task_update_time
            "#,
        )
        .bind(task_id as i64)
        .bind(name)
        .bind(description)
        .bind(&keywords)
        .bind(priority as i32)
        .bind(deadline)
        .bind("Active")
        .bind(task_create_time)
        .bind(leader_id as i64)
        .bind(team_id.map(|v| v as i64))
        .fetch_one(pool)
        .await?;

        tracing::info!("创建任务成功: task_id = {}", task_id);

        Ok(Self::row_to_task(result)?)
    }

    pub async fn get_task_by_id(pool: &PgPool, task_id: u64) -> Result<Option<Task>> {
        let result = sqlx::query(
            r#"
            SELECT task_id, task_name, task_description, task_keywords, task_priority, task_deadline, task_complete_time, task_status, task_create_time, task_leader_id, task_team_id, task_update_time
            FROM tasks
            WHERE task_id = $1
            "#,
        )
        .bind(task_id as i64)
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Self::row_to_task(row)?)),
            None => Ok(None),
        }
    }

    pub async fn list_tasks(pool: &PgPool, filter: TaskFilter) -> Result<Vec<Task>> {
        let mut conditions = Vec::new();
        let mut param_count = 1usize;

        if filter.leader_id.is_some() {
            conditions.push(format!("task_leader_id = ${}", param_count));
            param_count += 1;
        }
        if filter.team_id.is_some() {
            conditions.push(format!("task_team_id = ${}", param_count));
            param_count += 1;
        }
        if filter.status.is_some() {
            conditions.push(format!("task_status = ${}", param_count));
            param_count += 1;
        }
        if filter.priority.is_some() {
            conditions.push(format!("task_priority = ${}", param_count));
            param_count += 1;
        }
        if filter.deadline_before.is_some() {
            conditions.push(format!("task_deadline <= ${}", param_count));
            param_count += 1;
        }
        if filter.deadline_after.is_some() {
            conditions.push(format!("task_deadline >= ${}", param_count));
            param_count += 1;
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let limit = filter.limit.unwrap_or(50);
        let offset = filter.offset.unwrap_or(0);

        let query = format!(
            r#"
            SELECT task_id, task_name, task_description, task_keywords, task_priority, task_deadline, task_complete_time, task_status, task_create_time, task_leader_id, task_team_id, task_update_time
            FROM tasks
            {}
            ORDER BY task_create_time DESC
            LIMIT {} OFFSET {}
            "#,
            where_clause, limit, offset
        );

        let mut row_result = sqlx::query(&query);

        if let Some(v) = filter.leader_id {
            row_result = row_result.bind(v as i64);
        }
        if let Some(v) = filter.team_id {
            row_result = row_result.bind(v as i64);
        }
        if let Some(v) = filter.status {
            row_result = row_result.bind(match v {
                TaskStatus::Active => "Active",
                TaskStatus::Completed => "Completed",
                TaskStatus::Paused => "Paused",
            });
        }
        if let Some(v) = filter.priority {
            row_result = row_result.bind(v as i32);
        }
        if let Some(v) = filter.deadline_before {
            row_result = row_result.bind(v);
        }
        if let Some(v) = filter.deadline_after {
            row_result = row_result.bind(v);
        }

        let rows = row_result.fetch_all(pool).await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(Self::row_to_task(row)?);
        }

        Ok(tasks)
    }

    pub async fn update_task(
        pool: &PgPool,
        task_id: u64,
        name: Option<&str>,
        description: Option<&str>,
        keywords: Option<Vec<String>>,
        priority: Option<u8>,
        deadline: Option<Option<i64>>,
        team_id: Option<Option<u64>>,
    ) -> Result<Option<Task>> {
        let mut updates = Vec::new();
        let mut param_count = 1usize;

        if name.is_some() {
            updates.push(format!("task_name = ${}", param_count));
            param_count += 1;
        }
        if description.is_some() {
            updates.push(format!("task_description = ${}", param_count));
            param_count += 1;
        }
        if keywords.is_some() {
            updates.push(format!("task_keywords = ${}", param_count));
            param_count += 1;
        }
        if priority.is_some() {
            updates.push(format!("task_priority = ${}", param_count));
            param_count += 1;
        }
        if deadline.is_some() {
            updates.push(format!("task_deadline = ${}", param_count));
            param_count += 1;
        }
        if team_id.is_some() {
            updates.push(format!("task_team_id = ${}", param_count));
            param_count += 1;
        }

        if updates.is_empty() {
            return Self::get_task_by_id(pool, task_id).await;
        }

        let task_update_time = chrono::Utc::now().timestamp();
        updates.push(format!("task_update_time = ${}", param_count));
        param_count += 1;

        let query = format!(
            "UPDATE tasks SET {} WHERE task_id = ${} RETURNING task_id, task_name, task_description, task_keywords, task_priority, task_deadline, task_complete_time, task_status, task_create_time, task_leader_id, task_team_id, task_update_time",
            updates.join(", "),
            param_count
        );

        let mut row_result = sqlx::query(&query).bind(task_id as i64);

        if let Some(v) = name {
            row_result = row_result.bind(v);
        }
        if let Some(v) = description {
            row_result = row_result.bind(v);
        }
        if let Some(v) = keywords {
            row_result = row_result.bind(v);
        }
        if let Some(v) = priority {
            row_result = row_result.bind(v as i32);
        }
        if let Some(v) = deadline {
            row_result = row_result.bind(v);
        }
        if let Some(v) = team_id {
            row_result = row_result.bind(v.map(|id| id as i64));
        }
        row_result = row_result.bind(task_update_time);

        let result = row_result.fetch_optional(pool).await?;

        match result {
            Some(row) => {
                tracing::info!("更新任务成功: task_id = {}", task_id);
                Ok(Some(Self::row_to_task(row)?))
            }
            None => Ok(None),
        }
    }

    pub async fn delete_task(pool: &PgPool, task_id: u64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM tasks WHERE task_id = $1")
            .bind(task_id as i64)
            .execute(pool)
            .await?;

        let affected = result.rows_affected();
        tracing::info!("删除任务: task_id = {}, affected = {}", task_id, affected);
        Ok(affected > 0)
    }

    pub async fn complete_task(pool: &PgPool, task_id: u64) -> Result<Option<Task>> {
        let now = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            r#"
            UPDATE tasks 
            SET task_status = 'Completed', task_complete_time = $1, task_update_time = $1
            WHERE task_id = $2
            RETURNING task_id, task_name, task_description, task_keywords, task_priority, task_deadline, task_complete_time, task_status, task_create_time, task_leader_id, task_team_id, task_update_time
            "#,
        )
        .bind(now)
        .bind(task_id as i64)
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => {
                tracing::info!("完成任务成功: task_id = {}", task_id);
                Ok(Some(Self::row_to_task(row)?))
            }
            None => Ok(None),
        }
    }

    fn row_to_task(row: sqlx::postgres::PgRow) -> Result<Task> {
        let task_id: i64 = row.get("task_id");
        let task_name: String = row.get("task_name");
        let task_description: Option<String> = row.get("task_description");
        let task_keywords: Vec<String> = row.get("task_keywords");
        let task_priority: i32 = row.get("task_priority");
        let task_deadline: Option<i64> = row.get("task_deadline");
        let task_complete_time: Option<i64> = row.get("task_complete_time");
        let task_status: String = row.get("task_status");
        let task_create_time: i64 = row.get("task_create_time");
        let task_leader_id: i64 = row.get("task_leader_id");
        let task_team_id: Option<i64> = row.get("task_team_id");
        let task_update_time: Option<i64> = row.get("task_update_time");

        let task_status = match task_status.as_str() {
            "Active" => TaskStatus::Active,
            "Completed" => TaskStatus::Completed,
            "Paused" => TaskStatus::Paused,
            _ => TaskStatus::Active,
        };

        Ok(Task {
            task_id: task_id as u64,
            task_name,
            task_description,
            task_keywords: task_keywords.into_iter().collect(),
            task_priority: task_priority as u8,
            task_deadline: task_deadline.unwrap_or(0),
            task_complete_time,
            task_status,
            task_create_time,
            task_leader_id: task_leader_id as u64,
            task_team_id: task_team_id.map(|v| v as u64),
            task_update_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_crud() {
        let pool = crate::db::pool::create_pool().await.unwrap();

        let random_suffix = chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default() % 1000000;
        let username = format!("taskuser_{}", random_suffix);
        let email = format!("task_{}@example.com", random_suffix);

        let user = crate::db::db_user_back::DbUser::create_user(
            &pool,
            &username,
            "TestPass123!",
            &email,
            "13800138001",
        )
        .await
        .unwrap();

        let task = DbTask::create_task(
            &pool,
            "测试任务",
            user.user_id,
            Some("任务描述"),
            Some(vec!["tag1".to_string(), "tag2".to_string()]),
            5,
            Some(chrono::Utc::now().timestamp() + 86400 * 7),
            None,
        )
        .await
        .unwrap();

        println!("创建任务: {:?}", task);

        let found = DbTask::get_task_by_id(&pool, task.task_id).await.unwrap();
        println!("查询任务: {:?}", found);

        let filter = TaskFilter {
            leader_id: Some(user.user_id),
            ..Default::default()
        };
        let list = DbTask::list_tasks(&pool, filter).await.unwrap();
        println!("任务列表: {:?}", list);

        let updated = DbTask::update_task(
            &pool,
            task.task_id,
            Some("更新后的任务"),
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        println!("更新任务: {:?}", updated);

        let completed = DbTask::complete_task(&pool, task.task_id).await.unwrap();
        println!("完成任务: {:?}", completed);

        let deleted = DbTask::delete_task(&pool, task.task_id).await.unwrap();
        println!("删除任务: {}", deleted);

        crate::db::db_user_back::DbUser::delete_user(&pool, user.user_id)
            .await
            .unwrap();
    }
}
