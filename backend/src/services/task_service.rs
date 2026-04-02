use crate::db::db_task::DbTask;
use crate::models::task::{Task, TaskStatus};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub task_name: String,
    pub task_description: Option<String>,
    pub task_keywords: Option<Vec<String>>,
    pub task_priority: Option<u8>,
    pub task_deadline: Option<i64>,
    pub task_team_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub task_name: Option<String>,
    pub task_description: Option<String>,
    pub task_keywords: Option<Vec<String>>,
    pub task_priority: Option<u8>,
    pub task_deadline: Option<Option<i64>>,
    pub task_status: Option<TaskStatus>,
    pub task_leader_id: Option<u64>,
    pub task_team_id: Option<Option<u64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskStatusRequest {
    pub task_status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskPriorityRequest {
    pub task_priority: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTasksQuery {
    pub status: Option<TaskStatus>,
    pub priority: Option<u8>,
    pub deadline_before: Option<i64>,
    pub deadline_after: Option<i64>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

pub struct TaskService;

impl TaskService {
    pub async fn create_task(
        pool: &PgPool,
        user_id: u64,
        request: CreateTaskRequest,
    ) -> Result<Task> {
        let keywords = request.task_keywords.unwrap_or_default().into_iter().collect::<HashSet<String>>();

        let task = DbTask::create_task(
            pool,
            &request.task_name,
            request.task_description.as_deref(),
            keywords,
            request.task_priority.unwrap_or(0),
            request.task_deadline,
            user_id,
            request.task_team_id,
        )
        .await?;

        Ok(task)
    }

    pub async fn get_task_by_id(pool: &PgPool, task_id: u64) -> Result<Option<Task>> {
        DbTask::get_task_by_id(pool, task_id).await
    }

    pub async fn list_tasks(
        pool: &PgPool,
        user_id: Option<u64>,
        team_id: Option<u64>,
        query: ListTasksQuery,
    ) -> Result<Vec<Task>> {
        DbTask::list_tasks(
            pool,
            user_id,
            team_id,
            query.status,
            query.priority,
            query.deadline_before,
            query.deadline_after,
            query.limit,
            query.offset,
        )
        .await
    }

    pub async fn update_task(
        pool: &PgPool,
        task_id: u64,
        request: UpdateTaskRequest,
    ) -> Result<Option<Task>> {
        DbTask::update_task(
            pool,
            task_id,
            request.task_name.as_deref(),
            request.task_description.as_deref(),
            request.task_keywords.map(|k| k.into_iter().collect::<HashSet<String>>()),
            request.task_priority,
            request.task_deadline,
            request.task_status,
            request.task_leader_id,
            request.task_team_id,
        )
        .await
    }

    pub async fn delete_task(pool: &PgPool, task_id: u64) -> Result<bool> {
        DbTask::delete_task(pool, task_id).await
    }

    pub async fn update_task_status(
        pool: &PgPool,
        task_id: u64,
        status: TaskStatus,
    ) -> Result<Option<Task>> {
        if status == TaskStatus::Completed {
            DbTask::complete_task(pool, task_id).await?;
        } else {
            DbTask::update_task(
                pool,
                task_id,
                None,
                None,
                None,
                None,
                None,
                Some(status),
                None,
                None,
            )
            .await?;
        }
        DbTask::get_task_by_id(pool, task_id).await
    }

    pub async fn update_task_priority(
        pool: &PgPool,
        task_id: u64,
        priority: u8,
    ) -> Result<Option<Task>> {
        DbTask::update_task(
            pool,
            task_id,
            None,
            None,
            None,
            Some(priority),
            None,
            None,
            None,
            None,
        )
        .await
    }
}
