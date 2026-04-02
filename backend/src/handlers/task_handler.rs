use salvo::oapi::extract::PathParam;
use salvo::prelude::*;

use crate::db::pool::create_pool;
use crate::services::task_service::{
    CreateTaskRequest, ListTasksQuery, TaskService, UpdateTaskPriorityRequest, UpdateTaskRequest,
    UpdateTaskStatusRequest,
};

#[endpoint]
pub async fn create_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = match depot.get::<i64>("user_id").ok() {
        Some(id) => *id as u64,
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "Unauthorized",
                "message": "User not authenticated"
            })));
            return;
        }
    };

    let request: CreateTaskRequest = match req.parse_json().await {
        Ok(req) => req,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Invalid request body",
                "message": e.to_string()
            })));
            return;
        }
    };

    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection failed",
                "message": e.to_string()
            })));
            return;
        }
    };

    match TaskService::create_task(&pool, user_id, request).await {
        Ok(task) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(serde_json::json!({
                "message": "Task created successfully",
                "task": serde_json::to_value(&task).unwrap_or_default()
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Failed to create task",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn get_task(task_id: PathParam<u64>, res: &mut Response) {
    let task_id: u64 = task_id.into_inner();

    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection failed",
                "message": e.to_string()
            })));
            return;
        }
    };

    match TaskService::get_task_by_id(&pool, task_id).await {
        Ok(Some(task)) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "task": serde_json::to_value(&task).unwrap_or_default()
            })));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "Task not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Failed to fetch task",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn list_tasks(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let user_id = match depot.get::<i64>("user_id").ok() {
        Some(id) => Some(*id as u64),
        None => None,
    };

    let query: ListTasksQuery = req.parse_queries().unwrap_or_default();

    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection failed",
                "message": e.to_string()
            })));
            return;
        }
    };

    match TaskService::list_tasks(&pool, user_id, None, query).await {
        Ok(tasks) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "tasks": tasks.iter().map(|t| serde_json::to_value(t).unwrap_or_default()).collect::<Vec<_>>()
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Failed to fetch tasks",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn update_task(task_id: PathParam<u64>, req: &mut Request, res: &mut Response) {
    let task_id: u64 = task_id.into_inner();

    let request: UpdateTaskRequest = match req.parse_json().await {
        Ok(req) => req,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Invalid request body",
                "message": e.to_string()
            })));
            return;
        }
    };

    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection failed",
                "message": e.to_string()
            })));
            return;
        }
    };

    match TaskService::update_task(&pool, task_id, request).await {
        Ok(Some(task)) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Task updated successfully",
                "task": serde_json::to_value(&task).unwrap_or_default()
            })));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "Task not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Failed to update task",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn delete_task(task_id: PathParam<u64>, res: &mut Response) {
    let task_id: u64 = task_id.into_inner();

    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection failed",
                "message": e.to_string()
            })));
            return;
        }
    };

    match TaskService::delete_task(&pool, task_id).await {
        Ok(true) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Task deleted successfully"
            })));
        }
        Ok(false) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "Task not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Failed to delete task",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn update_task_status(task_id: PathParam<u64>, req: &mut Request, res: &mut Response) {
    let task_id: u64 = task_id.into_inner();

    let request: UpdateTaskStatusRequest = match req.parse_json().await {
        Ok(req) => req,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Invalid request body",
                "message": e.to_string()
            })));
            return;
        }
    };

    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection failed",
                "message": e.to_string()
            })));
            return;
        }
    };

    match TaskService::update_task_status(&pool, task_id, request.task_status).await {
        Ok(Some(task)) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Task status updated successfully",
                "task": serde_json::to_value(&task).unwrap_or_default()
            })));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "Task not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Failed to update task status",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn update_task_priority(task_id: PathParam<u64>, req: &mut Request, res: &mut Response) {
    let task_id: u64 = task_id.into_inner();

    let request: UpdateTaskPriorityRequest = match req.parse_json().await {
        Ok(req) => req,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Invalid request body",
                "message": e.to_string()
            })));
            return;
        }
    };

    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection failed",
                "message": e.to_string()
            })));
            return;
        }
    };

    match TaskService::update_task_priority(&pool, task_id, request.task_priority).await {
        Ok(Some(task)) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Task priority updated successfully",
                "task": serde_json::to_value(&task).unwrap_or_default()
            })));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "Task not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Failed to update task priority",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn get_task_logs(task_id: PathParam<u64>, res: &mut Response) {
    let _task_id: u64 = task_id.into_inner();

    res.status_code(StatusCode::OK);
    res.render(Json(serde_json::json!({
        "message": "Task logs endpoint - to be implemented",
        "logs": []
    })));
}
