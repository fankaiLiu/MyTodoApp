pub mod task_store;
pub mod team_store;
pub mod theme_store;
pub mod user_store;

use crate::api::ApiClient;
use leptos::prelude::*;

pub struct Stores {
    pub theme: theme_store::ThemeStore,
    pub user: user_store::UserStore,
    pub task: task_store::TaskStore,
    pub team: team_store::TeamStore,
    pub api: ApiClient,
}

pub fn create_stores() -> Stores {
    Stores {
        theme: theme_store::create_theme_store(),
        user: user_store::create_user_store(),
        task: task_store::create_task_store(),
        team: team_store::create_team_store(),
        api: ApiClient::new(None),
    }
}

pub fn provide_stores(stores: Stores) {
    provide_context(stores.theme);
    provide_context(stores.user);
    provide_context(stores.task);
    provide_context(stores.team);
    provide_context(stores.api);
}

pub fn use_theme_store() -> theme_store::ThemeStore {
    use_context::<theme_store::ThemeStore>().expect("ThemeStore not found in context.")
}

pub fn use_user_store() -> user_store::UserStore {
    use_context::<user_store::UserStore>().expect("UserStore not found in context.")
}

pub fn use_task_store() -> task_store::TaskStore {
    use_context::<task_store::TaskStore>().expect("TaskStore not found in context.")
}

pub fn use_team_store() -> team_store::TeamStore {
    use_context::<team_store::TeamStore>().expect("TeamStore not found in context.")
}

pub fn use_api_client() -> ApiClient {
    use_context::<ApiClient>().expect("ApiClient not found in context.")
}

fn get_local_storage() -> Option<web_sys::Storage> {
    document()
        .default_view()
        .and_then(|w| w.local_storage().ok().flatten())
}

pub fn set_local_storage_item(key: &str, value: &str) {
    if let Some(storage) = get_local_storage() {
        let _ = storage.set_item(key, value);
    }
}

pub fn get_local_storage_item(key: &str) -> Option<String> {
    get_local_storage().and_then(|s| s.get_item(key).ok().flatten())
}

pub fn remove_local_storage_item(key: &str) {
    if let Some(storage) = get_local_storage() {
        let _ = storage.remove_item(key);
    }
}
