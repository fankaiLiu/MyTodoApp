use salvo::prelude::*;

use crate::handlers::user_handler;
use crate::middleware;

pub fn user_router() -> Router {
    let auth_middleware = middleware::auth::auth_check;

    Router::with_path("api/users")
        .push(Router::with_path("register").post(user_handler::register))
        .push(Router::with_path("login").post(user_handler::login))
        .push(
            Router::with_path("{user_id}")
                .hoop(auth_middleware)
                .get(user_handler::get_user)
                .push(Router::with_path("password").put(user_handler::change_password))
                .push(Router::with_path("settings").put(user_handler::update_settings))
                .push(Router::with_path("teams").get(user_handler::get_user_teams))
                .push(Router::with_path("logs").get(user_handler::get_user_logs))
                .put(user_handler::update_user),
        )
}
