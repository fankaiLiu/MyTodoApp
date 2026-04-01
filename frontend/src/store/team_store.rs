use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamVisibility {
    Public,
    Private,
}

impl Default for TeamVisibility {
    fn default() -> Self {
        Self::Private
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamStatus {
    Active,
    Closed,
}

impl Default for TeamStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InviteStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamSettings {
    pub team_description: Option<String>,
    pub team_visibility: TeamVisibility,
    pub team_status: TeamStatus,
    pub team_avatar: Option<String>,
    pub team_member_limit: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Team {
    pub team_id: u64,
    pub team_name: String,
    pub team_leader_id: u64,
    pub team_members: Vec<TeamMember>,
    pub team_create_time: i64,
    pub sub_team_ids: Vec<u64>,
    pub team_settings: TeamSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMember {
    pub user_id: u64,
    pub level: u8,
    pub join_time: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoinRequest {
    pub request_id: u64,
    pub team_id: u64,
    pub user_id: u64,
    pub request_time: i64,
    pub status: RequestStatus,
    pub review_time: Option<i64>,
    pub reviewer_id: Option<u64>,
    pub review_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamInvite {
    pub invite_id: u64,
    pub team_id: u64,
    pub inviter_id: u64,
    pub invitee_id: Option<Vec<u64>>,
    pub create_time: i64,
    pub expire_time: i64,
    pub status: InviteStatus,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TeamListState {
    pub teams: Vec<Team>,
    pub active_team_id: Option<u64>,
    pub join_requests: Vec<JoinRequest>,
    pub invites: Vec<TeamInvite>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct TeamStore {
    pub state: ReadSignal<TeamListState>,
    pub set_state: WriteSignal<TeamListState>,
}

impl TeamStore {
    pub fn set_teams(&self, teams: Vec<Team>) {
        let mut state = self.state.get();
        state.teams = teams;
        state.is_loading = false;
        state.error = None;
        self.set_state.set(state);
    }

    pub fn set_loading(&self, loading: bool) {
        let mut state = self.state.get();
        state.is_loading = loading;
        if loading {
            state.error = None;
        }
        self.set_state.set(state);
    }

    pub fn set_error(&self, error: String) {
        let mut state = self.state.get();
        state.error = Some(error);
        state.is_loading = false;
        self.set_state.set(state);
    }

    pub fn add_team(&self, team: Team) {
        let mut state = self.state.get();
        state.teams.push(team);
        self.set_state.set(state);
    }

    pub fn update_team(&self, team_id: u64, updated: Team) {
        let mut state = self.state.get();
        if let Some(pos) = state.teams.iter().position(|t| t.team_id == team_id) {
            state.teams[pos] = updated;
            self.set_state.set(state);
        }
    }

    pub fn remove_team(&self, team_id: u64) {
        let mut state = self.state.get();
        state.teams.retain(|t| t.team_id != team_id);
        if state.active_team_id == Some(team_id) {
            state.active_team_id = None;
        }
        self.set_state.set(state);
    }

    pub fn set_active_team(&self, team_id: Option<u64>) {
        let mut state = self.state.get();
        state.active_team_id = team_id;
        self.set_state.set(state);
    }

    pub fn active_team(&self) -> Option<Team> {
        let state = self.state.get();
        state
            .active_team_id
            .and_then(|id| state.teams.iter().find(|t| t.team_id == id).cloned())
    }

    pub fn add_member(&self, team_id: u64, member: TeamMember) {
        let mut state = self.state.get();
        if let Some(team) = state.teams.iter_mut().find(|t| t.team_id == team_id) {
            team.team_members.push(member);
            self.set_state.set(state);
        }
    }

    pub fn remove_member(&self, team_id: u64, user_id: u64) {
        let mut state = self.state.get();
        if let Some(team) = state.teams.iter_mut().find(|t| t.team_id == team_id) {
            team.team_members.retain(|m| m.user_id != user_id);
            self.set_state.set(state);
        }
    }

    pub fn update_member_role(&self, team_id: u64, user_id: u64, level: u8) {
        let mut state = self.state.get();
        if let Some(team) = state.teams.iter_mut().find(|t| t.team_id == team_id) {
            if let Some(member) = team.team_members.iter_mut().find(|m| m.user_id == user_id) {
                member.level = level;
                self.set_state.set(state);
            }
        }
    }

    pub fn set_join_requests(&self, requests: Vec<JoinRequest>) {
        let mut state = self.state.get();
        state.join_requests = requests;
        self.set_state.set(state);
    }

    pub fn set_invites(&self, invites: Vec<TeamInvite>) {
        let mut state = self.state.get();
        state.invites = invites;
        self.set_state.set(state);
    }

    pub fn update_request_status(&self, request_id: u64, status: RequestStatus) {
        let mut state = self.state.get();
        if let Some(req) = state
            .join_requests
            .iter_mut()
            .find(|r| r.request_id == request_id)
        {
            req.status = status;
            self.set_state.set(state);
        }
    }

    pub fn update_invite_status(&self, invite_id: u64, status: InviteStatus) {
        let mut state = self.state.get();
        if let Some(invite) = state.invites.iter_mut().find(|i| i.invite_id == invite_id) {
            invite.status = status;
            self.set_state.set(state);
        }
    }

    pub fn my_teams(&self, user_id: u64) -> Vec<Team> {
        let state = self.state.get();
        state
            .teams
            .into_iter()
            .filter(|t| {
                t.team_leader_id == user_id || t.team_members.iter().any(|m| m.user_id == user_id)
            })
            .collect()
    }
}

pub fn create_team_store() -> TeamStore {
    let (state, set_state) = signal(TeamListState::default());
    TeamStore { state, set_state }
}
