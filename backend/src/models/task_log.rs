/*
    任务操作日志
*/
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log_TaskLog {
    pub log_id: u64,
    pub task_id: u64,
    pub operator_id: u64,
    pub action: TaskLogAction,
    pub old_value: Option<String>, // 变更前的值
    pub new_value: Option<String>, // 变更后的值
    pub details: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskLogAction {
    Created,         // 任务创建
    Updated,         // 任务更新
    Deleted,         // 任务删除
    StatusChanged,   // 状态变更
    PriorityChanged, // 优先级变更
    DeadlineChanged, // 截止时间变更
    LeaderChanged,   // 负责人变更
    TeamChanged,     // 所属团队变更
    CommentAdded,    // 添加评论
    AttachmentAdded, // 添加附件
}
