use serde::{Deserialize, Serialize};
use crate::api::{ApiClient, ApiResult};
use crate::store::team_store::{Team, TeamMember, JoinRequest, TeamInvite};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub team_name: String,
    pub team_description: Option<String>,
    pub team_visibility: Option<String>,
    pub team_member_limit: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_limit: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: u64,
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInviteRequest {
    pub invitee_ids: Vec<u64>,
    pub expire_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJoinRequest {
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateJoinRequestStatus {
    pub status: String,
    pub review_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamListResponse {
    pub teams: Vec<Team>,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamResponse {
    pub team: Team,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembersResponse {
    pub members: Vec<TeamMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitesResponse {
    pub invites: Vec<TeamInvite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRequestsResponse {
    pub requests: Vec<JoinRequest>,
}

pub async fn create_team(client: &ApiClient, req: &CreateTeamRequest) -> ApiResult<Team> {
    let resp: TeamResponse = client.post("/api/teams", req).await?;
    Ok(resp.team)
}

pub async fn get_team(client: &ApiClient, team_id: u64) -> ApiResult<Team> {
    let path = format!("/api/teams/{}", team_id);
    let resp: TeamResponse = client.get(&path).await?;
    Ok(resp.team)
}

pub async fn list_teams(client: &ApiClient) -> ApiResult<Vec<Team>> {
    let resp: TeamListResponse = client.get("/api/teams").await?;
    Ok(resp.teams)
}

pub async fn update_team(client: &ApiClient, team_id: u64, req: &UpdateTeamRequest) -> ApiResult<Team> {
    let path = format!("/api/teams/{}", team_id);
    let resp: TeamResponse = client.put(&path, req).await?;
    Ok(resp.team)
}

pub async fn delete_team(client: &ApiClient, team_id: u64) -> ApiResult<()> {
    let path = format!("/api/teams/{}", team_id);
    #[derive(Deserialize)]
    struct DeleteResponse {
        message: String,
    }
    let _: DeleteResponse = client.delete(&path).await?;
    Ok(())
}

pub async fn add_member(client: &ApiClient, team_id: u64, req: &AddMemberRequest) -> ApiResult<TeamMember> {
    let path = format!("/api/teams/{}/members", team_id);
    #[derive(Deserialize)]
    struct MemberResponse {
        member: TeamMember,
    }
    let resp: MemberResponse = client.post(&path, req).await?;
    Ok(resp.member)
}

pub async fn remove_member(client: &ApiClient, team_id: u64, user_id: u64) -> ApiResult<()> {
    let path = format!("/api/teams/{}/members/{}", team_id, user_id);
    #[derive(Deserialize)]
    struct DeleteResponse {
        message: String,
    }
    let _: DeleteResponse = client.delete(&path).await?;
    Ok(())
}

pub async fn update_member_role(
    client: &ApiClient,
    team_id: u64,
    user_id: u64,
    req: &UpdateRoleRequest,
) -> ApiResult<TeamMember> {
    let path = format!("/api/teams/{}/members/{}/role", team_id, user_id);
    #[derive(Deserialize)]
    struct MemberResponse {
        member: TeamMember,
    }
    let resp: MemberResponse = client.put(&path, req).await?;
    Ok(resp.member)
}

pub async fn get_members(client: &ApiClient, team_id: u64) -> ApiResult<Vec<TeamMember>> {
    let path = format!("/api/teams/{}/members", team_id);
    let resp: MembersResponse = client.get(&path).await?;
    Ok(resp.members)
}

pub async fn create_invite(
    client: &ApiClient,
    team_id: u64,
    req: &CreateInviteRequest,
) -> ApiResult<TeamInvite> {
    let path = format!("/api/teams/{}/invites", team_id);
    #[derive(Deserialize)]
    struct InviteResponse {
        invite: TeamInvite,
    }
    let resp: InviteResponse = client.post(&path, req).await?;
    Ok(resp.invite)
}

pub async fn create_join_request(
    client: &ApiClient,
    team_id: u64,
    req: &CreateJoinRequest,
) -> ApiResult<JoinRequest> {
    let path = format!("/api/teams/{}/join-requests", team_id);
    #[derive(Deserialize)]
    struct RequestResponse {
        request: JoinRequest,
    }
    let resp: RequestResponse = client.post(&path, req).await?;
    Ok(resp.request)
}

pub async fn update_join_request_status(
    client: &ApiClient,
    team_id: u64,
    request_id: u64,
    req: &UpdateJoinRequestStatus,
) -> ApiResult<JoinRequest> {
    let path = format!("/api/teams/{}/join-requests/{}", team_id, request_id);
    #[derive(Deserialize)]
    struct RequestResponse {
        request: JoinRequest,
    }
    let resp: RequestResponse = client.put(&path, req).await?;
    Ok(resp.request)
}

pub async fn get_team_logs(client: &ApiClient, team_id: u64) -> ApiResult<Vec<serde_json::Value>> {
    let path = format!("/api/teams/{}/logs", team_id);
    #[derive(Deserialize)]
    struct LogsResponse {
        logs: Vec<serde_json::Value>,
    }
    let resp: LogsResponse = client.get(&path).await?;
    Ok(resp.logs)
}
