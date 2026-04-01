use crate::store::{get_local_storage_item, remove_local_storage_item, set_local_storage_item};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const USER_KEY: &str = "todo_user";
const TOKEN_KEY: &str = "todo_token";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserProfile {
    pub user_id: u64,
    #[serde(rename = "user_username")]
    pub username: String,
    #[serde(rename = "user_email")]
    pub email: String,
    #[serde(rename = "user_phone")]
    pub phone: String,
    #[serde(rename = "user_avatar")]
    pub avatar: Option<String>,
    #[serde(rename = "user_description")]
    pub description: Option<String>,
    #[serde(rename = "user_reg_time")]
    pub reg_time: i64,
    #[serde(rename = "user_last_login_time")]
    pub last_login_time: Option<i64>,
    #[serde(rename = "user_teams")]
    pub team_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserState {
    pub is_authenticated: bool,
    pub profile: Option<UserProfile>,
    pub token: Option<String>,
}

#[derive(Clone)]
pub struct UserStore {
    pub state: ReadSignal<UserState>,
    pub set_state: WriteSignal<UserState>,
}

impl UserStore {
    pub fn login(&self, token: String, profile: UserProfile) {
        let mut state = self.state.get();
        state.is_authenticated = true;
        state.token = Some(token.clone());
        state.profile = Some(profile);
        self.set_state.set(state);

        set_local_storage_item(TOKEN_KEY, &token);
        if let Ok(json) = serde_json::to_string(&self.state.get().profile) {
            set_local_storage_item(USER_KEY, &json);
        }
    }

    pub fn logout(&self) {
        let state = UserState::default();
        self.set_state.set(state);

        remove_local_storage_item(TOKEN_KEY);
        remove_local_storage_item(USER_KEY);
    }

    pub fn update_profile(&self, profile: UserProfile) {
        let mut state = self.state.get();
        state.profile = Some(profile.clone());
        self.set_state.set(state);

        if let Ok(json) = serde_json::to_string(&Some(profile)) {
            set_local_storage_item(USER_KEY, &json);
        }
    }

    pub fn update_token(&self, token: String) {
        let mut state = self.state.get();
        state.token = Some(token.clone());
        self.set_state.set(state);

        set_local_storage_item(TOKEN_KEY, &token);
    }

    pub fn is_authenticated(&self) -> bool {
        self.state.get().is_authenticated
    }

    pub fn token(&self) -> Option<String> {
        self.state.get().token
    }

    pub fn profile(&self) -> Option<UserProfile> {
        self.state.get().profile
    }

    pub fn user_id(&self) -> Option<u64> {
        self.state.get().profile.map(|p| p.user_id)
    }
}

pub fn create_user_store() -> UserStore {
    let token = get_local_storage_item(TOKEN_KEY);
    let profile_json = get_local_storage_item(USER_KEY);
    let profile = profile_json.and_then(|j| serde_json::from_str::<UserProfile>(&j).ok());

    let is_authenticated = token.is_some() && profile.is_some();

    let initial = UserState {
        is_authenticated,
        profile,
        token,
    };

    let (state, set_state) = signal(initial);

    UserStore { state, set_state }
}
