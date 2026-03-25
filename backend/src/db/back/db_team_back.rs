use crate::models::team::{
    InviteStatus, JoinRequest, RequestStatus, Team, TeamInvite, TeamMember, TeamSettings,
    TeamStatus, TeamVisibility,
};
use crate::utils::id_generator;
use anyhow::Result;
use sqlx::{PgPool, Row};

pub struct DbTeam;

#[derive(Debug, Clone, Default)]
pub struct TeamFilter {
    pub leader_id: Option<u64>,
    pub member_user_id: Option<u64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl DbTeam {
    // 基础 CRUD 函数

    pub async fn create_team(
        pool: &PgPool,
        name: &str,
        leader_id: u64,
        description: Option<&str>,
        visibility: TeamVisibility,
        member_limit: u16,
    ) -> Result<Team> {
        let team_id = crate::utils::id_generator::generate_team_id();
        let team_create_time = chrono::Utc::now().timestamp();
        let sub_team_ids: Vec<i64> = vec![];
        let team_settings = TeamSettings {
            team_description: description.map(|s| s.to_string()),
            team_visibility: visibility,
            team_status: TeamStatus::Active,
            team_avatar: None,
            team_member_limit: member_limit,
        };

        let result = sqlx::query(
            r#"
            INSERT INTO teams (team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings
            "#,
        )
        .bind(team_id as i64)
        .bind(name)
        .bind(leader_id as i64)
        .bind(team_create_time)
        .bind(&sub_team_ids[..])
        .bind(serde_json::to_value(&team_settings)?)
        .fetch_one(pool)
        .await?;

        tracing::info!("创建团队成功: team_id = {}", team_id);

        Ok(Self::row_to_team(result)?)
    }

    pub async fn get_team_by_id(pool: &PgPool, team_id: u64) -> Result<Option<Team>> {
        let result = sqlx::query(
            r#"
            SELECT team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings
            FROM teams
            WHERE team_id = $1
            "#,
        )
        .bind(team_id as i64)
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Self::row_to_team(row)?)),
            None => Ok(None),
        }
    }

    pub async fn list_teams(pool: &PgPool, filter: TeamFilter) -> Result<Vec<Team>> {
        let mut conditions = Vec::new();
        let mut param_count = 1usize;

        if filter.leader_id.is_some() {
            conditions.push(format!("team_leader_id = ${}", param_count));
            param_count += 1;
        }
        if filter.member_user_id.is_some() {
            // 需要连接 team_members 表
            conditions.push(format!(
                "team_id IN (SELECT team_id FROM team_members WHERE user_id = ${})",
                param_count
            ));
            param_count += 1;
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let limit = filter.limit.unwrap_or(50);
        let offset = filter.offset.unwrap_or(0);

        let query = format!(
            r#"
            SELECT team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings
            FROM teams
            {}
            ORDER BY team_create_time DESC
            LIMIT {} OFFSET {}
            "#,
            where_clause, limit, offset
        );

        let mut row_result = sqlx::query(&query);

        if let Some(v) = filter.leader_id {
            row_result = row_result.bind(v as i64);
        }
        if let Some(v) = filter.member_user_id {
            row_result = row_result.bind(v as i64);
        }

        let rows = row_result.fetch_all(pool).await?;

        let mut teams = Vec::new();
        for row in rows {
            teams.push(Self::row_to_team(row)?);
        }

        Ok(teams)
    }

    pub async fn update_team(
        pool: &PgPool,
        team_id: u64,
        name: Option<&str>,
        description: Option<&str>,
        visibility: Option<TeamVisibility>,
        member_limit: Option<u16>,
    ) -> Result<Option<Team>> {
        // 先获取当前团队设置
        let current_team = Self::get_team_by_id(pool, team_id).await?;
        if current_team.is_none() {
            return Ok(None);
        }
        let current_team = current_team.unwrap();
        let mut settings = current_team.team_settings;

        let mut updates = Vec::new();
        let mut param_count = 1usize;

        if name.is_some() {
            updates.push(format!("team_name = ${}", param_count));
            param_count += 1;
        }
        if description.is_some() {
            settings.team_description = description.map(|s| s.to_string());
        }
        if visibility.is_some() {
            settings.team_visibility = visibility.unwrap();
        }
        if member_limit.is_some() {
            settings.team_member_limit = member_limit.unwrap();
        }

        // 更新 team_settings
        updates.push(format!("team_settings = ${}", param_count));
        param_count += 1;

        if updates.is_empty() {
            return Self::get_team_by_id(pool, team_id).await;
        }

        let query = format!(
            "UPDATE teams SET {} WHERE team_id = ${} RETURNING team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings",
            updates.join(", "),
            param_count
        );

        let mut row_result = sqlx::query(&query);

        if let Some(v) = name {
            row_result = row_result.bind(v);
        }
        row_result = row_result.bind(serde_json::to_value(&settings)?);
        row_result = row_result.bind(team_id as i64);

        let result = row_result.fetch_optional(pool).await?;

        match result {
            Some(row) => {
                tracing::info!("更新团队成功: team_id = {}", team_id);
                Ok(Some(Self::row_to_team(row)?))
            }
            None => Ok(None),
        }
    }

    pub async fn delete_team(pool: &PgPool, team_id: u64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM teams WHERE team_id = $1")
            .bind(team_id as i64)
            .execute(pool)
            .await?;

        let affected = result.rows_affected();
        tracing::info!("删除团队: team_id = {}, affected = {}", team_id, affected);
        Ok(affected > 0)
    }

    // 成员管理函数将在后续实现

    // 辅助函数：将数据库行转换为 Team 结构体
    fn row_to_team(row: sqlx::postgres::PgRow) -> Result<Team> {
        let team_id: i64 = row.get("team_id");
        let team_name: String = row.get("team_name");
        let team_leader_id: i64 = row.get("team_leader_id");
        let team_create_time: i64 = row.get("team_create_time");
        let sub_team_ids: Vec<i64> = row.get("sub_team_ids");
        let team_settings: serde_json::Value = row.get("team_settings");

        let team_settings: TeamSettings = serde_json::from_value(team_settings).unwrap_or_default();

        Ok(Team {
            team_id: team_id as u64,
            team_name,
            team_leader_id: team_leader_id as u64,
            team_members: vec![], // 需要额外查询填充
            team_create_time,
            sub_team_ids: sub_team_ids.into_iter().map(|v| v as u64).collect(),
            team_settings,
        })
    }

    // 成员管理函数

    pub async fn add_team_member(
        pool: &PgPool,
        team_id: u64,
        user_id: u64,
        level: u8,
    ) -> Result<bool> {
        let join_time = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            r#"
            INSERT INTO team_members (team_id, user_id, level, join_time)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (team_id, user_id) DO UPDATE SET level = EXCLUDED.level
            "#,
        )
        .bind(team_id as i64)
        .bind(user_id as i64)
        .bind(level as i32)
        .bind(join_time)
        .execute(pool)
        .await?;

        tracing::info!(
            "添加团队成员: team_id = {}, user_id = {}, level = {}",
            team_id,
            user_id,
            level
        );
        Ok(result.rows_affected() > 0)
    }

    pub async fn remove_team_member(pool: &PgPool, team_id: u64, user_id: u64) -> Result<bool> {
        let result = sqlx::query(
            r#"
            DELETE FROM team_members WHERE team_id = $1 AND user_id = $2
            "#,
        )
        .bind(team_id as i64)
        .bind(user_id as i64)
        .execute(pool)
        .await?;

        tracing::info!("移除团队成员: team_id = {}, user_id = {}", team_id, user_id);
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_team_members(pool: &PgPool, team_id: u64) -> Result<Vec<TeamMember>> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, level, join_time FROM team_members WHERE team_id = $1
            "#,
        )
        .bind(team_id as i64)
        .fetch_all(pool)
        .await?;

        let mut members = Vec::new();
        for row in rows {
            let user_id: i64 = row.get("user_id");
            let level: i32 = row.get("level");
            let join_time: i64 = row.get("join_time");
            members.push(TeamMember {
                user_id: user_id as u64,
                level: level as u8,
                join_time,
            });
        }

        Ok(members)
    }

    pub async fn update_member_role(
        pool: &PgPool,
        team_id: u64,
        user_id: u64,
        new_level: u8,
    ) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE team_members SET level = $1 WHERE team_id = $2 AND user_id = $3
            "#,
        )
        .bind(new_level as i32)
        .bind(team_id as i64)
        .bind(user_id as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            "更新成员角色: team_id = {}, user_id = {}, new_level = {}",
            team_id,
            user_id,
            new_level
        );
        Ok(result.rows_affected() > 0)
    }

    pub async fn check_team_membership(pool: &PgPool, team_id: u64, user_id: u64) -> Result<bool> {
        let result = sqlx::query(
            r#"
            SELECT 1 FROM team_members WHERE team_id = $1 AND user_id = $2
            "#,
        )
        .bind(team_id as i64)
        .bind(user_id as i64)
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }

    // 邀请管理函数

    pub async fn create_team_invite(
        pool: &PgPool,
        team_id: u64,
        inviter_id: u64,
        invitee_ids: Vec<u64>,
        expire_hours: u32,
    ) -> Result<TeamInvite> {
        let create_time = chrono::Utc::now().timestamp();
        let expire_time = create_time + (expire_hours as i64 * 3600);
        let status = InviteStatus::Pending;
        let invitee_ids_i64: Vec<i64> = invitee_ids.iter().map(|&v| v as i64).collect();

        let result = sqlx::query(
            r#"
            INSERT INTO team_invites (team_id, inviter_id, invitee_ids, create_time, expire_time, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING team_id, inviter_id, invitee_ids, create_time, expire_time, status
            "#,
        )
        .bind(team_id as i64)
        .bind(inviter_id as i64)
        .bind(&invitee_ids_i64[..])
        .bind(create_time)
        .bind(expire_time)
        .bind(match status {
            InviteStatus::Pending => "Pending",
            InviteStatus::Approved => "Approved",
            InviteStatus::Rejected => "Rejected",
        })
        .fetch_one(pool)
        .await?;

        tracing::info!(
            "创建团队邀请: team_id = {}, inviter_id = {}, invitee_count = {}",
            team_id,
            inviter_id,
            invitee_ids.len()
        );
        Ok(Self::row_to_team_invite(result)?)
    }

    pub async fn get_team_invites(
        pool: &PgPool,
        team_id: u64,
        status_filter: Option<InviteStatus>,
    ) -> Result<Vec<TeamInvite>> {
        let mut conditions = vec!["team_id = $1".to_string()];
        let mut param_count = 2;
        if let Some(status) = &status_filter {
            conditions.push(format!("status = ${}", param_count));
            param_count += 1;
        }
        let where_clause = conditions.join(" AND ");

        let query = format!(
            r#"
            SELECT team_id, inviter_id, invitee_ids, create_time, expire_time, status
            FROM team_invites
            WHERE {}
            ORDER BY create_time DESC
            "#,
            where_clause
        );

        let mut row_result = sqlx::query(&query).bind(team_id as i64);
        if let Some(status) = status_filter {
            row_result = row_result.bind(match status {
                InviteStatus::Pending => "Pending",
                InviteStatus::Approved => "Approved",
                InviteStatus::Rejected => "Rejected",
            });
        }

        let rows = row_result.fetch_all(pool).await?;
        let mut invites = Vec::new();
        for row in rows {
            invites.push(Self::row_to_team_invite(row)?);
        }
        Ok(invites)
    }

    pub async fn update_team_invite_status(
        pool: &PgPool,
        team_id: u64,
        inviter_id: u64,
        create_time: i64,
        new_status: InviteStatus,
    ) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE team_invites SET status = $1
            WHERE team_id = $2 AND inviter_id = $3 AND create_time = $4
            "#,
        )
        .bind(match new_status {
            InviteStatus::Pending => "Pending",
            InviteStatus::Approved => "Approved",
            InviteStatus::Rejected => "Rejected",
        })
        .bind(team_id as i64)
        .bind(inviter_id as i64)
        .bind(create_time)
        .execute(pool)
        .await?;

        tracing::info!(
            "更新团队邀请状态: team_id = {}, inviter_id = {}, new_status = {:?}",
            team_id,
            inviter_id,
            new_status
        );
        Ok(result.rows_affected() > 0)
    }

    // 辅助函数：将数据库行转换为 TeamInvite 结构体
    fn row_to_team_invite(row: sqlx::postgres::PgRow) -> Result<TeamInvite> {
        let team_id: i64 = row.get("team_id");
        let inviter_id: i64 = row.get("inviter_id");
        let invitee_ids: Vec<i64> = row.get("invitee_ids");
        let create_time: i64 = row.get("create_time");
        let expire_time: i64 = row.get("expire_time");
        let status: String = row.get("status");

        let status = match status.as_str() {
            "Pending" => InviteStatus::Pending,
            "Approved" => InviteStatus::Approved,
            "Rejected" => InviteStatus::Rejected,
            _ => InviteStatus::Pending,
        };

        Ok(TeamInvite {
            invite_id: 0, // 表中无 invite_id，暂设为 0
            team_id: team_id as u64,
            inviter_id: inviter_id as u64,
            invitee_id: Some(invitee_ids.into_iter().map(|v| v as u64).collect()),
            create_time,
            expire_time,
            status,
        })
    }

    // 加入申请函数

    pub async fn create_join_request(
        pool: &PgPool,
        team_id: u64,
        user_id: u64,
    ) -> Result<JoinRequest> {
        let request_id = crate::utils::id_generator::generate_join_request_id();
        let request_time = chrono::Utc::now().timestamp();
        let status = RequestStatus::Pending;

        let result = sqlx::query(
            r#"
            INSERT INTO join_requests (request_id, team_id, user_id, request_time, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING request_id, team_id, user_id, request_time, status, review_time, reviewer_id, review_message
            "#,
        )
        .bind(request_id as i64)
        .bind(team_id as i64)
        .bind(user_id as i64)
        .bind(request_time)
        .bind(match status {
            RequestStatus::Pending => "Pending",
            RequestStatus::Approved => "Approved",
            RequestStatus::Rejected => "Rejected",
        })
        .fetch_one(pool)
        .await?;

        tracing::info!(
            "创建加入申请: request_id = {}, team_id = {}, user_id = {}",
            request_id,
            team_id,
            user_id
        );
        Ok(Self::row_to_join_request(result)?)
    }

    pub async fn get_join_requests(
        pool: &PgPool,
        team_id: Option<u64>,
        user_id: Option<u64>,
        status_filter: Option<RequestStatus>,
    ) -> Result<Vec<JoinRequest>> {
        let mut conditions = Vec::new();
        let mut param_count = 1usize;

        if let Some(tid) = team_id {
            conditions.push(format!("team_id = ${}", param_count));
            param_count += 1;
        }
        if let Some(uid) = user_id {
            conditions.push(format!("user_id = ${}", param_count));
            param_count += 1;
        }
        if let Some(status) = &status_filter {
            conditions.push(format!("status = ${}", param_count));
            param_count += 1;
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let query = format!(
            r#"
            SELECT request_id, team_id, user_id, request_time, status, review_time, reviewer_id, review_message
            FROM join_requests
            {}
            ORDER BY request_time DESC
            "#,
            where_clause
        );

        let mut row_result = sqlx::query(&query);
        if let Some(tid) = team_id {
            row_result = row_result.bind(tid as i64);
        }
        if let Some(uid) = user_id {
            row_result = row_result.bind(uid as i64);
        }
        if let Some(status) = status_filter {
            row_result = row_result.bind(match status {
                RequestStatus::Pending => "Pending",
                RequestStatus::Approved => "Approved",
                RequestStatus::Rejected => "Rejected",
            });
        }

        let rows = row_result.fetch_all(pool).await?;
        let mut requests = Vec::new();
        for row in rows {
            requests.push(Self::row_to_join_request(row)?);
        }
        Ok(requests)
    }

    pub async fn update_join_request_status(
        pool: &PgPool,
        request_id: u64,
        new_status: RequestStatus,
        reviewer_id: Option<u64>,
        review_message: Option<&str>,
    ) -> Result<bool> {
        let review_time = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            r#"
            UPDATE join_requests
            SET status = $1, review_time = $2, reviewer_id = $3, review_message = $4
            WHERE request_id = $5
            "#,
        )
        .bind(match new_status {
            RequestStatus::Pending => "Pending",
            RequestStatus::Approved => "Approved",
            RequestStatus::Rejected => "Rejected",
        })
        .bind(review_time)
        .bind(reviewer_id.map(|v| v as i64))
        .bind(review_message)
        .bind(request_id as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            "更新加入申请状态: request_id = {}, new_status = {:?}",
            request_id,
            new_status
        );
        Ok(result.rows_affected() > 0)
    }

    // 辅助函数：将数据库行转换为 JoinRequest 结构体
    fn row_to_join_request(row: sqlx::postgres::PgRow) -> Result<JoinRequest> {
        let request_id: i64 = row.get("request_id");
        let team_id: i64 = row.get("team_id");
        let user_id: i64 = row.get("user_id");
        let request_time: i64 = row.get("request_time");
        let status: String = row.get("status");
        let review_time: Option<i64> = row.get("review_time");
        let reviewer_id: Option<i64> = row.get("reviewer_id");
        let review_message: Option<String> = row.get("review_message");

        let status = match status.as_str() {
            "Pending" => RequestStatus::Pending,
            "Approved" => RequestStatus::Approved,
            "Rejected" => RequestStatus::Rejected,
            _ => RequestStatus::Pending,
        };

        Ok(JoinRequest {
            request_id: request_id as u64,
            team_id: team_id as u64,
            user_id: user_id as u64,
            request_time,
            status,
            review_time,
            reviewer_id: reviewer_id.map(|v| v as u64),
            review_message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_team_crud() {
        let pool = crate::db::pool::create_pool().await.unwrap();

        // 创建用户作为团队负责人
        let user = crate::db::db_user_back::DbUser::create_user(
            &pool,
            "teamuser",
            "TestPass123!",
            "team@example.com",
            "13800138002",
        )
        .await
        .unwrap();

        // 创建团队
        let team = DbTeam::create_team(
            &pool,
            "测试团队",
            user.user_id,
            Some("团队描述"),
            TeamVisibility::Private,
            50,
        )
        .await
        .unwrap();

        println!("创建团队: {:?}", team);

        // 获取团队
        let found = DbTeam::get_team_by_id(&pool, team.team_id).await.unwrap();
        println!("查询团队: {:?}", found);
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.team_name, "测试团队");
        assert_eq!(found.team_leader_id, user.user_id);

        // 列表筛选
        let filter = TeamFilter {
            leader_id: Some(user.user_id),
            ..Default::default()
        };
        let list = DbTeam::list_teams(&pool, filter).await.unwrap();
        println!("团队列表: {:?}", list);
        assert!(!list.is_empty());

        // 更新团队
        let updated = DbTeam::update_team(
            &pool,
            team.team_id,
            Some("更新后的团队名"),
            Some("新的描述"),
            Some(TeamVisibility::Public),
            Some(100),
        )
        .await
        .unwrap();
        println!("更新团队: {:?}", updated);
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.team_name, "更新后的团队名");
        match updated.team_settings.team_visibility {
            TeamVisibility::Public => {}
            _ => panic!("可见性未更新"),
        }

        // 成员管理
        let added = DbTeam::add_team_member(&pool, team.team_id, user.user_id, 2)
            .await
            .unwrap();
        assert!(added);
        let members = DbTeam::get_team_members(&pool, team.team_id).await.unwrap();
        println!("团队成员: {:?}", members);
        assert!(!members.is_empty());

        let is_member = DbTeam::check_team_membership(&pool, team.team_id, user.user_id)
            .await
            .unwrap();
        assert!(is_member);

        let updated_role = DbTeam::update_member_role(&pool, team.team_id, user.user_id, 3)
            .await
            .unwrap();
        assert!(updated_role);

        let removed = DbTeam::remove_team_member(&pool, team.team_id, user.user_id)
            .await
            .unwrap();
        assert!(removed);

        // 邀请管理
        let invite =
            DbTeam::create_team_invite(&pool, team.team_id, user.user_id, vec![user.user_id], 24)
                .await
                .unwrap();
        println!("创建邀请: {:?}", invite);
        assert_eq!(invite.team_id, team.team_id);

        let invites = DbTeam::get_team_invites(&pool, team.team_id, None)
            .await
            .unwrap();
        println!("团队邀请列表: {:?}", invites);
        assert!(!invites.is_empty());

        let updated_status = DbTeam::update_team_invite_status(
            &pool,
            team.team_id,
            user.user_id,
            invite.create_time,
            InviteStatus::Approved,
        )
        .await
        .unwrap();
        assert!(updated_status);

        // 加入申请
        let request = DbTeam::create_join_request(&pool, team.team_id, user.user_id)
            .await
            .unwrap();
        println!("创建加入申请: {:?}", request);
        assert_eq!(request.team_id, team.team_id);

        let requests =
            DbTeam::get_join_requests(&pool, Some(team.team_id), Some(user.user_id), None)
                .await
                .unwrap();
        println!("加入申请列表: {:?}", requests);
        assert!(!requests.is_empty());

        let updated_request = DbTeam::update_join_request_status(
            &pool,
            request.request_id,
            RequestStatus::Approved,
            Some(user.user_id),
            Some("欢迎加入"),
        )
        .await
        .unwrap();
        assert!(updated_request);

        // 删除团队
        let deleted = DbTeam::delete_team(&pool, team.team_id).await.unwrap();
        println!("删除团队: {}", deleted);
        assert!(deleted);

        // 清理用户
        crate::db::db_user_back::DbUser::delete_user(&pool, user.user_id)
            .await
            .unwrap();
    }
}
