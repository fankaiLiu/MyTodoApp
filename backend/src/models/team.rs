/*
借鉴： warframe 氏族功能
    团队结构体：
    team_id: 雪花ID，全局唯一，u64类型
    team_name: 团队名称
    team_leader_id: 团队负责人ID，雪花ID，u64类型(user中user_id)
    team_members: 团队成员ID列表，雪花ID，Vec<u64>类型
    team_create_time: 团队创建时间，Unix时间戳
    team_description: 团队描述，可选
    team_visibility: 团队可见性，公开或私有
    team_status: 团队状态: 运行中或已关闭
    以下为后续版本升级可选字段
    team_member_limit: 团队总成员上限，u16类型(0~65535)
    sub_team_member_limit: 子团队成员上限，u16类型(0~65535)
    sub_teams_name: 子团队名称列表，Vec<String>类型
    sub_teams_id: 子团队ID列表，雪花ID，Vec<u64>类型
    sub_team_description: 子团队描述，可选
    新层级(Option)：组织（Organization）层级
    用于管理组织下的团队，每个组织即为一个公司整体，或者集团。
    如果时间允许，后续版本可以考虑添加组织层级。
*/

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    // 团队ID，雪花ID，全局唯一，u64类型
    pub team_id: u64,
    // 团队名称
    pub team_name: String,
    // 团队负责人ID，雪花ID，u64类型(user中user_id)
    pub team_leader_id: u64,
    // 团队成员ID列表，雪花ID，Vec<TeamMember>类型
    pub team_members: Vec<TeamMember>,
    // 团队创建时间，Unix时间戳
    pub team_create_time: i64,
    // 子团队ID列表，Vec<u64>类型
    pub sub_team_ids: Vec<u64>,
    // 团队设置：包括团队描述、可见性、状态、头像、总成员上限
    pub team_settings: TeamSettings,
}

// 团队设置结构体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TeamSettings {
    // 团队描述，可选
    pub team_description: Option<String>,
    // 团队可见性，公开或私有/仅邀请
    pub team_visibility: TeamVisibility,
    // 团队状态: 运行中或已关闭
    pub team_status: TeamStatus,
    // 团队头像，可选
    pub team_avatar: Option<String>,
    // 团队总成员上限，u16类型(0~65535)
    pub team_member_limit: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SubTeam {
    // 子团队ID，雪花ID，全局唯一，u64类型
    pub sub_team_id: u64,
    // 子团队名称
    pub sub_team_name: String,
    // 子团队负责人ID，雪花ID，u64类型(user中user_id)
    pub sub_team_leader_id: u64,
    // 子团队成员ID列表，雪花ID，HashSet<u64>类型
    pub sub_team_members: HashSet<u64>,
    // 子团队创建时间，Unix时间戳
    pub sub_team_create_time: i64,
    // 子团队描述，可选
    pub sub_team_description: Option<String>,
    // 子团队所属团队ID，雪花ID，u64类型
    pub team_id: u64,
}

// 团队成员结构体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub user_id: u64,
    // 团队成员等级
    pub level: u8,
    // 加入时间，Unix时间戳
    pub join_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamVisibility {
    Public,  // 公开
    Private, // 私有 / 仅邀请 / 默认私有
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamStatus {
    Active, // 运行中 / 默认运行中
    Closed, // 已关闭
}

impl Default for TeamStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl Default for TeamVisibility {
    fn default() -> Self {
        Self::Private
    }
}

// 团队加入申请结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRequest {
    pub request_id: u64,       // 申请ID
    pub team_id: u64,          // 目标团队ID
    pub user_id: u64,          // 申请人ID
    pub request_time: i64,     // 申请时间
    pub status: RequestStatus, // 申请状态
    // pub message: Option<String>,        // 申请留言
    pub review_time: Option<i64>,       // 审批时间
    pub reviewer_id: Option<u64>,       // 审批人ID
    pub review_message: Option<String>, // 审批回复
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestStatus {
    Pending,  // 待审批
    Approved, // 已通过
    Rejected, // 已拒绝
}

// 团队邀请结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInvite {
    pub invite_id: u64,               // 邀请ID
    pub team_id: u64,                 // 团队ID
    pub inviter_id: u64,              // 邀请人ID
    pub invitee_id: Option<Vec<u64>>, // 被邀请人ID(可一次性邀请多个)
    // pub invite_code: String,     // 邀请码 (可分享的链接)
    pub create_time: i64, // 邀请时间
    pub expire_time: i64, // 过期时间
    // pub max_uses: Option<u32>, // 最大使用次数
    // pub used_count: u32,       // 已使用次数
    pub status: InviteStatus, // 邀请状态
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InviteStatus {
    Pending,  // 待审批
    Approved, // 已通过
    Rejected, // 已拒绝
}
