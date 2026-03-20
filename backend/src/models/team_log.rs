use serde::{Deserialize, Serialize};

/*
 * 团队操作日志
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log_TeamLog {
    pub log_id: u64,
    pub team_id: u64,
    pub operator_id: u64,           // 操作者ID
    pub action: LogAction,          // 操作类型
    pub target_type: String,        // 目标类型 (task, member, team, etc.)
    pub target_id: Option<u64>,     // 目标ID
    pub details: String,            // 详情描述
    pub created_at: i64,            // 操作时间
    pub ip_address: Option<String>, // IP地址
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogAction {
    // 成员相关
    MemberJoined,      // 成员加入
    MemberLeft,        // 成员离开
    MemberRemoved,     // 成员被移除
    MemberRoleChanged, // 成员角色变更

    // 团队相关
    TeamCreated, // 团队创建
    TeamUpdated, // 团队信息更新
    TeamClosed,  // 团队关闭

    // 子团队
    SubTeamCreated, // 子团队创建
    SubTeamDeleted, // 子团队删除

    // 审批相关
    RequestApproved, // 申请通过
    RequestRejected, // 申请拒绝

    // 任务相关
    TaskCreated,   // 任务创建
    TaskCompleted, // 任务完成
    TaskDeleted,   // 任务删除
}
