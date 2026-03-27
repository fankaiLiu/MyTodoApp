use crate::models::team::{
    JoinRequest, RequestStatus, Team, TeamInvite, TeamMember, TeamSettings, TeamStatus,
    TeamVisibility,
};
use crate::utils::id_generator::generate_team_id;
use anyhow::Result;
use sqlx::{PgPool, Row};

pub struct DbTeam;

impl DbTeam {
    pub async fn create_team(pool: &PgPool, team_name: &str, team_leader_id: u64) -> Result<Team> {
        let team_id = generate_team_id();
        let team_create_time = chrono::Utc::now().timestamp();

        let sub_team_ids: Vec<i64> = vec![];
        let sub_team_ids_json = serde_json::to_value(&sub_team_ids).unwrap();
        let team_settings = TeamSettings::default();

        let result = sqlx::query(
            r#"
            INSERT INTO teams (team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings
            "#,
        )
        .bind(team_id as i64)
        .bind(team_name)
        .bind(team_leader_id as i64)
        .bind(team_create_time)
        .bind(sub_team_ids_json)
        .bind(serde_json::to_value(&team_settings)?)
        .fetch_one(pool)
        .await?;

        tracing::info!(
            "创建团队成功: team_id = {}, team_name = {}",
            team_id,
            team_name
        );

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

    pub async fn list_teams(
        pool: &PgPool,
        leader_id: Option<u64>,
        user_id: Option<u64>,
    ) -> Result<Vec<Team>> {
        let mut query = String::from(
            "SELECT team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings FROM teams WHERE 1=1",
        );
        let mut param_count = 0;

        if leader_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND team_leader_id = ${}", param_count));
        }

        if user_id.is_some() {
            param_count += 1;
            query.push_str(&format!(
                " AND team_id IN (SELECT team_id FROM team_members WHERE user_id = ${})",
                param_count
            ));
        }

        query.push_str(" ORDER BY team_create_time DESC");

        let mut sql_query = sqlx::query(&query);

        if let Some(id) = leader_id {
            sql_query = sql_query.bind(id as i64);
        }
        if let Some(id) = user_id {
            sql_query = sql_query.bind(id as i64);
        }

        let rows = sql_query.fetch_all(pool).await?;

        let mut teams = Vec::new();
        for row in rows {
            teams.push(Self::row_to_team(row)?);
        }

        Ok(teams)
    }

    pub async fn update_team(
        pool: &PgPool,
        team_id: u64,
        team_name: Option<&str>,
        team_description: Option<&str>,
        team_visibility: Option<TeamVisibility>,
        team_status: Option<TeamStatus>,
        team_avatar: Option<&str>,
        team_member_limit: Option<u16>,
    ) -> Result<Option<Team>> {
        let team = match Self::get_team_by_id(pool, team_id).await? {
            Some(t) => t,
            None => return Ok(None),
        };

        let new_team_name = team_name.unwrap_or(&team.team_name);
        let new_description = team_description.or(team.team_settings.team_description.as_deref());
        let new_visibility = team_visibility.unwrap_or(team.team_settings.team_visibility);
        let new_status = team_status.unwrap_or(team.team_settings.team_status);
        let new_avatar = team_avatar.or(team.team_settings.team_avatar.as_deref());
        let new_member_limit = team_member_limit.unwrap_or(team.team_settings.team_member_limit);

        let new_settings = TeamSettings {
            team_description: new_description.map(|s| s.to_string()),
            team_visibility: new_visibility,
            team_status: new_status,
            team_avatar: new_avatar.map(|s| s.to_string()),
            team_member_limit: new_member_limit,
        };

        let result = sqlx::query(
            r#"
            UPDATE teams 
            SET team_name = $1, team_settings = $2 
            WHERE team_id = $3 
            RETURNING team_id, team_name, team_leader_id, team_create_time, sub_team_ids, team_settings
            "#,
        )
        .bind(new_team_name)
        .bind(serde_json::to_value(&new_settings)?)
        .bind(team_id as i64)
        .fetch_optional(pool)
        .await?;

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
            ON CONFLICT (team_id, user_id) DO NOTHING
            "#,
        )
        .bind(team_id as i64)
        .bind(user_id as i64)
        .bind(level as i32)
        .bind(join_time)
        .execute(pool)
        .await?;

        let affected = result.rows_affected();
        tracing::info!(
            "添加团队成员: team_id = {}, user_id = {}, level = {}, affected = {}",
            team_id,
            user_id,
            level,
            affected
        );
        Ok(affected > 0)
    }

    pub async fn remove_team_member(pool: &PgPool, team_id: u64, user_id: u64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM team_members WHERE team_id = $1 AND user_id = $2")
            .bind(team_id as i64)
            .bind(user_id as i64)
            .execute(pool)
            .await?;

        let affected = result.rows_affected();
        tracing::info!(
            "移除团队成员: team_id = {}, user_id = {}, affected = {}",
            team_id,
            user_id,
            affected
        );
        Ok(affected > 0)
    }

    pub async fn get_team_members(pool: &PgPool, team_id: u64) -> Result<Vec<TeamMember>> {
        let result = sqlx::query(
            r#"
            SELECT user_id, level, join_time
            FROM team_members
            WHERE team_id = $1
            ORDER BY join_time ASC
            "#,
        )
        .bind(team_id as i64)
        .fetch_all(pool)
        .await?;

        let mut members = Vec::new();
        for row in result {
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
        level: u8,
    ) -> Result<bool> {
        let result =
            sqlx::query("UPDATE team_members SET level = $1 WHERE team_id = $2 AND user_id = $3")
                .bind(level as i32)
                .bind(team_id as i64)
                .bind(user_id as i64)
                .execute(pool)
                .await?;

        let affected = result.rows_affected();
        tracing::info!(
            "更新成员角色: team_id = {}, user_id = {}, level = {}, affected = {}",
            team_id,
            user_id,
            level,
            affected
        );
        Ok(affected > 0)
    }

    pub async fn check_team_membership(pool: &PgPool, team_id: u64, user_id: u64) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM team_members WHERE team_id = $1 AND user_id = $2")
            .bind(team_id as i64)
            .bind(user_id as i64)
            .fetch_optional(pool)
            .await?;

        Ok(result.is_some())
    }

    pub async fn create_team_invite(
        pool: &PgPool,
        team_id: u64,
        inviter_id: u64,
        invitee_ids: Vec<u64>,
        expire_hours: i64,
    ) -> Result<TeamInvite> {
        let create_time = chrono::Utc::now().timestamp();
        let expire_time = create_time + (expire_hours * 3600);

        let invitee_ids_json = serde_json::to_value(&invitee_ids).unwrap();

        let result = sqlx::query(
            r#"
            INSERT INTO team_invites (team_id, inviter_id, invitee_ids, create_time, expire_time, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING team_id, inviter_id, invitee_ids, create_time, expire_time, status
            "#,
        )
        .bind(team_id as i64)
        .bind(inviter_id as i64)
        .bind(invitee_ids_json)
        .bind(create_time)
        .bind(expire_time)
        .bind("Pending")
        .fetch_one(pool)
        .await?;

        tracing::info!(
            "创建团队邀请: team_id = {}, inviter_id = {}",
            team_id,
            inviter_id
        );

        Ok(Self::row_to_team_invite(result)?)
    }

    pub async fn get_team_invites(
        pool: &PgPool,
        team_id: Option<u64>,
        status: Option<&str>,
    ) -> Result<Vec<TeamInvite>> {
        let mut query = String::from(
            "SELECT team_id, inviter_id, invitee_ids, create_time, expire_time, status FROM team_invites WHERE 1=1",
        );
        let mut param_count = 0;

        if team_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND team_id = ${}", param_count));
        }

        if status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
        }

        query.push_str(" ORDER BY create_time DESC");

        let mut sql_query = sqlx::query(&query);

        if let Some(id) = team_id {
            sql_query = sql_query.bind(id as i64);
        }
        if let Some(s) = status {
            sql_query = sql_query.bind(s);
        }

        let rows = sql_query.fetch_all(pool).await?;

        let mut invites = Vec::new();
        for row in rows {
            invites.push(Self::row_to_team_invite(row)?);
        }

        Ok(invites)
    }

    pub async fn update_team_invite_status(
        pool: &PgPool,
        team_id: u64,
        invite_id: u64,
        status: &str,
    ) -> Result<bool> {
        let result =
            sqlx::query("UPDATE team_invites SET status = $1 WHERE team_id = $2 AND team_id = $2")
                .bind(status)
                .bind(team_id as i64)
                .execute(pool)
                .await?;

        let affected = result.rows_affected();
        tracing::info!(
            "更新邀请状态: team_id = {}, status = {}, affected = {}",
            team_id,
            status,
            affected
        );
        Ok(affected > 0)
    }

    pub async fn create_join_request(
        pool: &PgPool,
        team_id: u64,
        user_id: u64,
    ) -> Result<JoinRequest> {
        let request_id = crate::utils::id_generator::generate_join_request_id();
        let request_time = chrono::Utc::now().timestamp();

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
        .bind("Pending")
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
        status: Option<&str>,
    ) -> Result<Vec<JoinRequest>> {
        let mut query = String::from(
            "SELECT request_id, team_id, user_id, request_time, status, review_time, reviewer_id, review_message FROM join_requests WHERE 1=1",
        );
        let mut param_count = 0;

        if team_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND team_id = ${}", param_count));
        }

        if user_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND user_id = ${}", param_count));
        }

        if status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
        }

        query.push_str(" ORDER BY request_time DESC");

        let mut sql_query = sqlx::query(&query);

        if let Some(id) = team_id {
            sql_query = sql_query.bind(id as i64);
        }
        if let Some(id) = user_id {
            sql_query = sql_query.bind(id as i64);
        }
        if let Some(s) = status {
            sql_query = sql_query.bind(s);
        }

        let rows = sql_query.fetch_all(pool).await?;

        let mut requests = Vec::new();
        for row in rows {
            requests.push(Self::row_to_join_request(row)?);
        }

        Ok(requests)
    }

    pub async fn update_join_request_status(
        pool: &PgPool,
        request_id: u64,
        status: &str,
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
        .bind(status)
        .bind(review_time)
        .bind(reviewer_id.map(|id| id as i64))
        .bind(review_message)
        .bind(request_id as i64)
        .execute(pool)
        .await?;

        let affected = result.rows_affected();
        tracing::info!(
            "更新申请状态: request_id = {}, status = {}, affected = {}",
            request_id,
            status,
            affected
        );
        Ok(affected > 0)
    }

    fn row_to_team(row: sqlx::postgres::PgRow) -> Result<Team> {
        let team_id: i64 = row.get("team_id");
        let team_name: String = row.get("team_name");
        let team_leader_id: i64 = row.get("team_leader_id");
        let team_create_time: i64 = row.get("team_create_time");
        let sub_team_ids: serde_json::Value = row.get("sub_team_ids");
        let team_settings: serde_json::Value = row.get("team_settings");

        let sub_team_ids: Vec<i64> = serde_json::from_value(sub_team_ids)
            .map_err(|e| anyhow::anyhow!("解析子团队ID失败: {}", e))?;
        let team_settings: TeamSettings = serde_json::from_value(team_settings)
            .map_err(|e| anyhow::anyhow!("解析团队设置失败: {}", e))?;

        Ok(Team {
            team_id: team_id as u64,
            team_name,
            team_leader_id: team_leader_id as u64,
            team_members: vec![], // 成员通过单独的查询获取，避免N+1查询问题
            team_create_time,
            sub_team_ids: sub_team_ids.into_iter().map(|v| v as u64).collect(),
            team_settings,
        })
    }

    fn row_to_team_invite(row: sqlx::postgres::PgRow) -> Result<TeamInvite> {
        let team_id: i64 = row.get("team_id");
        let inviter_id: i64 = row.get("inviter_id");
        let invitee_ids: serde_json::Value = row.get("invitee_ids");
        let create_time: i64 = row.get("create_time");
        let expire_time: i64 = row.get("expire_time");
        let status: String = row.get("status");

        let invitee_ids: Vec<i64> = serde_json::from_value(invitee_ids).unwrap_or_default();
        let status = match status.as_str() {
            "Approved" => crate::models::team::InviteStatus::Approved,
            "Rejected" => crate::models::team::InviteStatus::Rejected,
            _ => crate::models::team::InviteStatus::Pending,
        };

        Ok(TeamInvite {
            invite_id: 0,
            team_id: team_id as u64,
            inviter_id: inviter_id as u64,
            invitee_id: Some(invitee_ids.into_iter().map(|v| v as u64).collect()),
            create_time,
            expire_time,
            status,
        })
    }

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

        let user = crate::db::db_user::DbUser::create_user(
            &pool,
            "test_team_leader",
            "TestPass123!",
            "test_team_leader@example.com",
            "13800000001",
        )
        .await
        .unwrap();

        let team = DbTeam::create_team(&pool, "Test Team", user.user_id)
            .await
            .unwrap();

        println!("创建团队: {:?}", team);
        assert_eq!(team.team_name, "Test Team");
        assert_eq!(team.team_leader_id, user.user_id);

        let found = DbTeam::get_team_by_id(&pool, team.team_id).await.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.team_id, team.team_id);
        assert_eq!(found.team_name, "Test Team");

        let updated = DbTeam::update_team(
            &pool,
            team.team_id,
            Some("Updated Team Name"),
            Some("Test description"),
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().team_name, "Updated Team Name");

        let deleted = DbTeam::delete_team(&pool, team.team_id).await.unwrap();
        assert!(deleted);

        crate::db::db_user::DbUser::delete_user(&pool, user.user_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_team_members() {
        let pool = crate::db::pool::create_pool().await.unwrap();

        let leader = crate::db::db_user::DbUser::create_user(
            &pool,
            "test_member_leader",
            "TestPass123!",
            "test_member_leader@example.com",
            "13800000002",
        )
        .await
        .unwrap();

        let member = crate::db::db_user::DbUser::create_user(
            &pool,
            "test_member_user",
            "TestPass123!",
            "test_member_user@example.com",
            "13800000003",
        )
        .await
        .unwrap();

        let team = DbTeam::create_team(&pool, "Member Test Team", leader.user_id)
            .await
            .unwrap();

        let add_result = DbTeam::add_team_member(&pool, team.team_id, member.user_id, 2)
            .await
            .unwrap();
        assert!(add_result);

        let is_member = DbTeam::check_team_membership(&pool, team.team_id, member.user_id)
            .await
            .unwrap();
        assert!(is_member);

        let members = DbTeam::get_team_members(&pool, team.team_id).await.unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].user_id, member.user_id);

        let role_updated = DbTeam::update_member_role(&pool, team.team_id, member.user_id, 3)
            .await
            .unwrap();
        assert!(role_updated);

        let remove_result = DbTeam::remove_team_member(&pool, team.team_id, member.user_id)
            .await
            .unwrap();
        assert!(remove_result);

        let is_not_member = DbTeam::check_team_membership(&pool, team.team_id, member.user_id)
            .await
            .unwrap();
        assert!(!is_not_member);

        DbTeam::delete_team(&pool, team.team_id).await.unwrap();
        crate::db::db_user::DbUser::delete_user(&pool, leader.user_id)
            .await
            .unwrap();
        crate::db::db_user::DbUser::delete_user(&pool, member.user_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_invite_and_request() {
        let pool = crate::db::pool::create_pool().await.unwrap();

        if let Some(u) =
            crate::db::db_user::DbUser::get_user_by_username(&pool, "test_invite_leader")
                .await
                .unwrap()
        {
            crate::db::db_user::DbUser::delete_user(&pool, u.user_id)
                .await
                .unwrap();
        }
        if let Some(u) =
            crate::db::db_user::DbUser::get_user_by_username(&pool, "test_invite_member")
                .await
                .unwrap()
        {
            crate::db::db_user::DbUser::delete_user(&pool, u.user_id)
                .await
                .unwrap();
        }

        let leader = crate::db::db_user::DbUser::create_user(
            &pool,
            "test_invite_leader",
            "TestPass123!",
            "test_invite_leader@example.com",
            "13800000004",
        )
        .await
        .unwrap();

        let member = crate::db::db_user::DbUser::create_user(
            &pool,
            "test_invite_member",
            "TestPass123!",
            "test_invite_member@example.com",
            "13800000005",
        )
        .await
        .unwrap();

        let team = DbTeam::create_team(&pool, "Invite Test Team", leader.user_id)
            .await
            .unwrap();

        let invite = DbTeam::create_team_invite(
            &pool,
            team.team_id,
            leader.user_id,
            vec![member.user_id],
            24,
        )
        .await
        .unwrap();

        println!("创建邀请: {:?}", invite);
        assert_eq!(invite.team_id, team.team_id);
        assert_eq!(invite.inviter_id, leader.user_id);

        let invites = DbTeam::get_team_invites(&pool, Some(team.team_id), None)
            .await
            .unwrap();
        assert!(!invites.is_empty());

        let request = DbTeam::create_join_request(&pool, team.team_id, member.user_id)
            .await
            .unwrap();

        println!("创建申请: {:?}", request);
        assert_eq!(request.team_id, team.team_id);
        assert_eq!(request.user_id, member.user_id);

        let requests = DbTeam::get_join_requests(&pool, Some(team.team_id), None, None)
            .await
            .unwrap();
        assert!(!requests.is_empty());

        let status_updated = DbTeam::update_join_request_status(
            &pool,
            request.request_id,
            "Approved",
            Some(leader.user_id),
            Some("Welcome!"),
        )
        .await
        .unwrap();
        assert!(status_updated);

        DbTeam::delete_team(&pool, team.team_id).await.unwrap();
        crate::db::db_user::DbUser::delete_user(&pool, leader.user_id)
            .await
            .unwrap();
        crate::db::db_user::DbUser::delete_user(&pool, member.user_id)
            .await
            .unwrap();
    }
}
