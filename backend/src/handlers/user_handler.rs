use salvo::oapi::extract::PathParam;
use salvo::prelude::*;

use crate::db::pool::create_pool;
use crate::services::user_service::{
    ChangePasswordRequest, LoginRequest, RegisterRequest, UpdateSettingsRequest, UpdateUserRequest,
    UserService,
};

#[endpoint]
pub async fn register(req: &mut Request, res: &mut Response) {
    let request: RegisterRequest = match req.parse_json().await {
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

    match UserService::register(&pool, request).await {
        Ok(response) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(serde_json::json!({
                "message": "Registration successful",
                "access_token": response.access_token,
                "refresh_token": response.refresh_token,
                "user": serde_json::to_value(&response.user).unwrap_or_default()
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Registration failed",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn login(req: &mut Request, res: &mut Response) {
    let request: LoginRequest = match req.parse_json().await {
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

    match UserService::login(&pool, request).await {
        Ok(response) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Login successful",
                "access_token": response.access_token,
                "refresh_token": response.refresh_token,
                "user": serde_json::to_value(&response.user).unwrap_or_default()
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "Login failed",
                "message": e.to_string()
            })));
        }
    }
}

fn check_auth(depot: &mut Depot, user_id: u64) -> bool {
    if let Some(requested_user_id) = depot.get::<i64>("user_id").ok() {
        *requested_user_id == user_id as i64
    } else {
        false
    }
}

#[endpoint]
pub async fn get_user(user_id: PathParam<u64>, depot: &mut Depot, res: &mut Response) {
    let user_id: u64 = user_id.into_inner();

    if !check_auth(depot, user_id) {
        res.status_code(StatusCode::FORBIDDEN);
        res.render(Json(serde_json::json!({
            "error": "Forbidden",
            "message": "You can only access your own profile"
        })));
        return;
    }

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

    match UserService::get_user_by_id(&pool, user_id).await {
        Ok(Some(user)) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "user": serde_json::to_value(&user).unwrap_or_default()
            })));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "User not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Failed to fetch user",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn update_user(
    user_id: PathParam<u64>,
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) {
    let user_id: u64 = user_id.into_inner();

    if !check_auth(depot, user_id) {
        res.status_code(StatusCode::FORBIDDEN);
        res.render(Json(serde_json::json!({
            "error": "Forbidden",
            "message": "You can only update your own profile"
        })));
        return;
    }

    let request: UpdateUserRequest = match req.parse_json().await {
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

    match UserService::update_user(&pool, user_id, request).await {
        Ok(Some(user)) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "User updated successfully",
                "user": serde_json::to_value(&user).unwrap_or_default()
            })));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "User not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Failed to update user",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn change_password(
    user_id: PathParam<u64>,
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) {
    let user_id: u64 = user_id.into_inner();

    if !check_auth(depot, user_id) {
        res.status_code(StatusCode::FORBIDDEN);
        res.render(Json(serde_json::json!({
            "error": "Forbidden",
            "message": "You can only change your own password"
        })));
        return;
    }

    let request: ChangePasswordRequest = match req.parse_json().await {
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

    match UserService::change_password(&pool, user_id, request).await {
        Ok(_) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Password changed successfully"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Failed to change password",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn update_settings(
    user_id: PathParam<u64>,
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) {
    let user_id: u64 = user_id.into_inner();

    if !check_auth(depot, user_id) {
        res.status_code(StatusCode::FORBIDDEN);
        res.render(Json(serde_json::json!({
            "error": "Forbidden",
            "message": "You can only update your own settings"
        })));
        return;
    }

    let request: UpdateSettingsRequest = match req.parse_json().await {
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

    match UserService::update_settings(&pool, user_id, request).await {
        Ok(Some(user)) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Settings updated successfully",
                "user": serde_json::to_value(&user).unwrap_or_default()
            })));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "User not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Failed to update settings",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn get_user_teams(user_id: PathParam<u64>, depot: &mut Depot, res: &mut Response) {
    let user_id: u64 = user_id.into_inner();

    if !check_auth(depot, user_id) {
        res.status_code(StatusCode::FORBIDDEN);
        res.render(Json(serde_json::json!({
            "error": "Forbidden",
            "message": "You can only access your own teams"
        })));
        return;
    }

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

    match UserService::get_user_teams(&pool, user_id).await {
        Ok(teams) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "teams": teams
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Failed to fetch teams",
                "message": e.to_string()
            })));
        }
    }
}

#[endpoint]
pub async fn get_user_logs(user_id: PathParam<u64>, depot: &mut Depot, res: &mut Response) {
    let user_id: u64 = user_id.into_inner();

    if !check_auth(depot, user_id) {
        res.status_code(StatusCode::FORBIDDEN);
        res.render(Json(serde_json::json!({
            "error": "Forbidden",
            "message": "You can only access your own logs"
        })));
        return;
    }

    res.status_code(StatusCode::OK);
    res.render(Json(serde_json::json!({
        "message": "User logs endpoint - to be implemented",
        "logs": []
    })));
}
