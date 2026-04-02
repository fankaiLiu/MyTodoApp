use salvo::prelude::*;

use crate::handlers::task_handler;
use crate::middleware;

pub fn task_router() -> Router {
    let auth_middleware = middleware::auth::auth_check;

    Router::with_path("api/tasks")
        .hoop(auth_middleware)
        .post(task_handler::create_task)
        .get(task_handler::list_tasks)
        .push(
            Router::with_path("{task_id}")
                .get(task_handler::get_task)
                .put(task_handler::update_task)
                .delete(task_handler::delete_task)
                .push(Router::with_path("status").put(task_handler::update_task_status))
                .push(Router::with_path("priority").put(task_handler::update_task_priority))
                .push(Router::with_path("logs").get(task_handler::get_task_logs)),
        )
}
