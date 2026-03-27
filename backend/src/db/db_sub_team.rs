use crate::models::team::SubTeam;
use anyhow::Result;
use sqlx::{PgPool, Row};

pub struct DbSubTeam;

impl DbSubTeam {
    pub async fn create_sub_team(
        pool: &PgPool,
        sub_team_name: &str,
        sub_team_leader_id: u64,
        team_id: u64,
        sub_team_description: Option<&str>,
    ) -> Result<SubTeam> {
        let sub_team_id = crate::utils::id_generator::generate_team_id();
        let sub_team_create_time = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            r#"
            INSERT INTO sub_teams (sub_team_id, sub_team_name, sub_team_leader_id, team_id, sub_team_create_time, sub_team_description)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING sub_team_id, sub_team_name, sub_team_leader_id, team_id, sub_team_create_time, sub_team_description
            "#,
        )
        .bind(sub_team_id as i64)
        .bind(sub_team_name)
        .bind(sub_team_leader_id as i64)
        .bind(team_id as i64)
        .bind(sub_team_create_time)
        .bind(sub_team_description)
        .fetch_one(pool)
        .await?;

        tracing::info!(
            "创建子团队成功: sub_team_id = {}, sub_team_name = {}",
            sub_team_id,
            sub_team_name
        );

        Ok(Self::row_to_sub_team(result)?)
    }

    pub async fn get_sub_team_by_id(pool: &PgPool, sub_team_id: u64) -> Result<Option<SubTeam>> {
        let result = sqlx::query(
            r#"
            SELECT sub_team_id, sub_team_name, sub_team_leader_id, team_id, sub_team_create_time, sub_team_description
            FROM sub_teams
            WHERE sub_team_id = $1
            "#,
        )
        .bind(sub_team_id as i64)
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => {
                let mut sub_team = Self::row_to_sub_team(row)?;

                // Load members from sub_team_members table
                let members = crate::db::db_sub_team_member::DbSubTeamMember::get_sub_team_members(
                    pool,
                    sub_team_id,
                )
                .await?;

                sub_team.sub_team_members = members;

                Ok(Some(sub_team))
            }
            None => Ok(None),
        }
    }

    pub async fn list_sub_teams(
        pool: &PgPool,
        team_id: Option<u64>,
        sub_team_leader_id: Option<u64>,
    ) -> Result<Vec<SubTeam>> {
        let mut query = String::from(
            "SELECT sub_team_id, sub_team_name, sub_team_leader_id, team_id, sub_team_create_time, sub_team_description FROM sub_teams WHERE 1=1",
        );
        let mut bind_idx = 1;

        if team_id.is_some() {
            query.push_str(&format!(" AND team_id = ${}", bind_idx));
            bind_idx += 1;
        }
        if sub_team_leader_id.is_some() {
            query.push_str(&format!(" AND sub_team_leader_id = ${}", bind_idx));
        }

        let mut sql_query = sqlx::query(&query);

        if let Some(tid) = team_id {
            sql_query = sql_query.bind(tid as i64);
        }
        if let Some(lid) = sub_team_leader_id {
            sql_query = sql_query.bind(lid as i64);
        }

        let rows = sql_query.fetch_all(pool).await?;
        let mut sub_teams = Vec::new();

        for row in rows {
            let mut sub_team = Self::row_to_sub_team(row)?;

            // Load members from sub_team_members table
            let members = crate::db::db_sub_team_member::DbSubTeamMember::get_sub_team_members(
                pool,
                sub_team.sub_team_id,
            )
            .await?;

            sub_team.sub_team_members = members;

            sub_teams.push(sub_team);
        }

        Ok(sub_teams)
    }

    pub async fn update_sub_team(
        pool: &PgPool,
        sub_team_id: u64,
        sub_team_name: Option<&str>,
        sub_team_leader_id: Option<u64>,
        sub_team_description: Option<&str>,
    ) -> Result<Option<SubTeam>> {
        let existing = Self::get_sub_team_by_id(pool, sub_team_id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let existing = existing.unwrap();
        let new_name = sub_team_name.unwrap_or(&existing.sub_team_name);
        let new_leader_id = sub_team_leader_id
            .map(|id| id as i64)
            .unwrap_or(existing.sub_team_leader_id as i64);
        let new_description = sub_team_description.or(existing.sub_team_description.as_deref());

        let result = sqlx::query(
            r#"
            UPDATE sub_teams
            SET sub_team_name = $1, sub_team_leader_id = $2, sub_team_description = $3
            WHERE sub_team_id = $4
            RETURNING sub_team_id, sub_team_name, sub_team_leader_id, team_id, sub_team_create_time, sub_team_description
            "#,
        )
        .bind(new_name)
        .bind(new_leader_id)
        .bind(new_description)
        .bind(sub_team_id as i64)
        .fetch_one(pool)
        .await?;

        tracing::info!("更新子团队成功: sub_team_id = {}", sub_team_id);

        let mut sub_team = Self::row_to_sub_team(result)?;

        // Load members from sub_team_members table
        let members =
            crate::db::db_sub_team_member::DbSubTeamMember::get_sub_team_members(pool, sub_team_id)
                .await?;

        sub_team.sub_team_members = members;

        Ok(Some(sub_team))
    }

    pub async fn delete_sub_team(pool: &PgPool, sub_team_id: u64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM sub_teams WHERE sub_team_id = $1")
            .bind(sub_team_id as i64)
            .execute(pool)
            .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            tracing::info!("删除子团队成功: sub_team_id = {}", sub_team_id);
        }

        Ok(deleted)
    }

    fn row_to_sub_team(row: sqlx::postgres::PgRow) -> Result<SubTeam> {
        Ok(SubTeam {
            sub_team_id: row.get::<i64, _>("sub_team_id") as u64,
            sub_team_name: row.get("sub_team_name"),
            sub_team_leader_id: row.get::<i64, _>("sub_team_leader_id") as u64,
            team_id: row.get::<i64, _>("team_id") as u64,
            sub_team_create_time: row.get("sub_team_create_time"),
            sub_team_description: row.get("sub_team_description"),
            sub_team_members: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sub_team_crud() {
        let pool = crate::db::pool::create_pool().await.unwrap();

        let timestamp = chrono::Utc::now().timestamp();
        let unique_suffix = timestamp.to_string();

        let leader = crate::db::db_user::DbUser::create_user(
            &pool,
            &format!("test_sub_team_leader_{}", unique_suffix),
            "TestPass123!",
            &format!("test_sub_team_leader_{}@example.com", unique_suffix),
            &format!("1380000{:04}", timestamp % 10000),
        )
        .await
        .unwrap();

        let team_leader = crate::db::db_user::DbUser::create_user(
            &pool,
            &format!("test_team_leader_for_sub_{}", unique_suffix),
            "TestPass123!",
            &format!("test_team_leader_for_sub_{}@example.com", unique_suffix),
            &format!("1380001{:04}", timestamp % 10000),
        )
        .await
        .unwrap();

        let team =
            crate::db::db_team::DbTeam::create_team(&pool, "Test Parent Team", team_leader.user_id)
                .await
                .unwrap();

        let sub_team = DbSubTeam::create_sub_team(
            &pool,
            "Test Sub Team",
            leader.user_id,
            team.team_id,
            Some("Test sub team description"),
        )
        .await
        .unwrap();

        println!("创建子团队: {:?}", sub_team);
        assert_eq!(sub_team.sub_team_name, "Test Sub Team");
        assert_eq!(sub_team.sub_team_leader_id, leader.user_id);
        assert_eq!(sub_team.team_id, team.team_id);

        let found = DbSubTeam::get_sub_team_by_id(&pool, sub_team.sub_team_id)
            .await
            .unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.sub_team_id, sub_team.sub_team_id);
        assert_eq!(found.sub_team_name, "Test Sub Team");

        let list = DbSubTeam::list_sub_teams(&pool, Some(team.team_id), None)
            .await
            .unwrap();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        let updated = DbSubTeam::update_sub_team(
            &pool,
            sub_team.sub_team_id,
            Some("Updated Sub Team Name"),
            None,
            Some("Updated description"),
        )
        .await
        .unwrap();
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().sub_team_name, "Updated Sub Team Name");

        let deleted = DbSubTeam::delete_sub_team(&pool, sub_team.sub_team_id)
            .await
            .unwrap();
        assert!(deleted);

        let found_after_delete = DbSubTeam::get_sub_team_by_id(&pool, sub_team.sub_team_id)
            .await
            .unwrap();
        assert!(found_after_delete.is_none());

        crate::db::db_team::DbTeam::delete_team(&pool, team.team_id)
            .await
            .unwrap();
        crate::db::db_user::DbUser::delete_user(&pool, leader.user_id)
            .await
            .unwrap();
        crate::db::db_user::DbUser::delete_user(&pool, team_leader.user_id)
            .await
            .unwrap();
    }
}
