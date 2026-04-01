use serde::{Deserialize, Serialize};
use crate::api::{ApiClient, ApiResult};
use crate::store::task_store::Task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub task_name: String,
    pub task_description: Option<String>,
    pub task_keywords: Vec<String>,
    pub task_priority: u8,
    pub task_deadline: Option<i64>,
    pub task_leader_id: u64,
    pub task_team_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_deadline: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub tasks: Vec<Task>,
    pub total: u32,
}

pub async fn create_task(client: &ApiClient, req: &CreateTaskRequest) -> ApiResult<Task> {
    client.post("/api/tasks", req).await
}

pub async fn get_task(client: &ApiClient, task_id: u64) -> ApiResult<Task> {
    let path = format!("/api/tasks/{}", task_id);
    #[derive(Deserialize)]
    struct TaskResponse {
        task: Task,
    }
    let resp: TaskResponse = client.get(&path).await?;
    Ok(resp.task)
}

pub async fn list_tasks(
    client: &ApiClient,
    page: u32,
    page_size: u32,
    status: Option<&str>,
    team_id: Option<u64>,
) -> ApiResult<TaskListResponse> {
    let mut path = format!("/api/tasks?page={}&page_size={}", page, page_size);
    if let Some(s) = status {
        path.push_str(&format!("&status={}", s));
    }
    if let Some(tid) = team_id {
        path.push_str(&format!("&team_id={}", tid));
    }
    client.get(&path).await
}

pub async fn update_task(client: &ApiClient, task_id: u64, req: &UpdateTaskRequest) -> ApiResult<Task> {
    let path = format!("/api/tasks/{}", task_id);
    #[derive(Deserialize)]
    struct TaskResponse {
        task: Task,
    }
    let resp: TaskResponse = client.put(&path, req).await?;
    Ok(resp.task)
}

pub async fn delete_task(client: &ApiClient, task_id: u64) -> ApiResult<()> {
    let path = format!("/api/tasks/{}", task_id);
    #[derive(Deserialize)]
    struct DeleteResponse {
        message: String,
    }
    let _: DeleteResponse = client.delete(&path).await?;
    Ok(())
}

pub async fn update_task_status(client: &ApiClient, task_id: u64, status: &str) -> ApiResult<Task> {
    let path = format!("/api/tasks/{}/status", task_id);
    #[derive(Serialize)]
    struct StatusReq {
        status: String,
    }
    #[derive(Deserialize)]
    struct TaskResponse {
        task: Task,
    }
    let resp: TaskResponse = client.put(&path, &StatusReq { status: status.to_string() }).await?;
    Ok(resp.task)
}

pub async fn update_task_priority(client: &ApiClient, task_id: u64, priority: u8) -> ApiResult<Task> {
    let path = format!("/api/tasks/{}/priority", task_id);
    #[derive(Serialize)]
    struct PriorityReq {
        priority: u8,
    }
    #[derive(Deserialize)]
    struct TaskResponse {
        task: Task,
    }
    let resp: TaskResponse = client.put(&path, &PriorityReq { priority }).await?;
    Ok(resp.task)
}

pub async fn get_task_logs(client: &ApiClient, task_id: u64) -> ApiResult<Vec<serde_json::Value>> {
    let path = format!("/api/tasks/{}/logs", task_id);
    #[derive(Deserialize)]
    struct LogsResponse {
        logs: Vec<serde_json::Value>,
    }
    let resp: LogsResponse = client.get(&path).await?;
    Ok(resp.logs)
}
