// use salvo::oapi::extract::*;
// use salvo::prelude::*;
// use nulid::Nulid;
use serde::{Deserialize, Serialize};
// use sonyflake::Sonyflake;

use crate::models::user_settings::UserSettings;

/*
    用户结构体：
    user_id: 用户ID，全局唯一，雪花ID
    user_username: 用户名
    user_password: 密码，哈希后存储
    user_email: 邮箱，全局唯一
    user_reg_time: 注册时间，Unix时间戳
    user_phone: 手机号，全局唯一
    user_teams: 加入了哪些团队
    user_last_login_time: 最后登录时间，Unix时间戳
    user_description: 用户描述(自我介绍/个人介绍)，可选
*/

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: u64,
    pub user_username: String,
    pub user_password: String,
    pub user_email: String,
    // 注册时间，Unix时间戳
    pub user_reg_time: i64,
    // 手机号
    pub user_phone: String,
    // 加入的团队ID列表
    pub user_teams: Vec<u64>,
    // 最后登录时间，Unix时间戳
    pub user_last_login_time: i64,
    // 用户描述(自我介绍/个人介绍)，可选
    pub user_description: Option<String>,
    pub user_settings: UserSettings,
    pub user_avatar: Option<String>,
    pub user_status: UserStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    Active,   // 进行中 / 默认运行中
    Inactive, // 已停用
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

// pub fn test_sonyflake_id() -> Result<u64, sonyflake::Error> {
//     let sf = Sonyflake::new()?;
//     let next_id = sf.next_id()?;
//     println!("sonyflake id: {}", next_id);
//     Ok(next_id)
// }

// pub fn test_nulid_id() -> Result<String, nulid::Error> {
//     let id = Nulid::new()?;
//     println!("nulid id: {}", id);
//     Ok(id.to_string())
// }
