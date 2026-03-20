/*
 * 用户操作日志->仅记录本地模式下的操作
 * 记录用户在系统中的操作，包括登录、注册、修改密码等。
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log_UserLog {
    pub log_id: u64,
    pub user_id: u64,
    pub action: UserLogAction,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserLogAction {
    Register,        // 注册
    Login,           // 登录
    Logout,          // 登出
    PasswordChanged, // 密码修改
    EmailUpdated,    // 邮箱更新
    PhoneUpdated,    // 手机号更新
    ProfileUpdated,  // 个人信息更新
    AvatarUpdated,   // 头像更新
}
