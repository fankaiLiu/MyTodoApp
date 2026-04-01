use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskStatus {
    Active,
    Completed,
    Paused,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Task {
    pub task_id: u64,
    pub task_name: String,
    pub task_description: Option<String>,
    pub task_keywords: HashSet<String>,
    pub task_priority: u8,
    pub task_deadline: Option<i64>,
    pub task_complete_time: Option<i64>,
    pub task_status: TaskStatus,
    pub task_create_time: i64,
    pub task_leader_id: u64,
    pub task_team_id: Option<u64>,
    pub task_update_time: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskFilters {
    pub status: Option<TaskStatus>,
    pub priority_min: Option<u8>,
    pub priority_max: Option<u8>,
    pub team_id: Option<u64>,
    pub assignee_id: Option<u64>,
    pub has_deadline: Option<bool>,
    pub search_query: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaginationState {
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
}

impl Default for PaginationState {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
            total: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskListState {
    pub tasks: Vec<Task>,
    pub filters: TaskFilters,
    pub pagination: PaginationState,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct TaskStore {
    pub state: ReadSignal<TaskListState>,
    pub set_state: WriteSignal<TaskListState>,
}

impl TaskStore {
    pub fn set_tasks(&self, tasks: Vec<Task>, total: u32) {
        let mut state = self.state.get();
        state.tasks = tasks;
        state.pagination.total = total;
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

    pub fn add_task(&self, task: Task) {
        let mut state = self.state.get();
        state.tasks.push(task);
        state.pagination.total += 1;
        self.set_state.set(state);
    }

    pub fn update_task(&self, task_id: u64, updated: Task) {
        let mut state = self.state.get();
        if let Some(pos) = state.tasks.iter().position(|t| t.task_id == task_id) {
            state.tasks[pos] = updated;
            self.set_state.set(state);
        }
    }

    pub fn remove_task(&self, task_id: u64) {
        let mut state = self.state.get();
        state.tasks.retain(|t| t.task_id != task_id);
        state.pagination.total = state.pagination.total.saturating_sub(1);
        self.set_state.set(state);
    }

    pub fn complete_task(&self, task_id: u64) {
        let mut state = self.state.get();
        if let Some(task) = state.tasks.iter_mut().find(|t| t.task_id == task_id) {
            task.task_status = TaskStatus::Completed;
            task.task_complete_time = Some(chrono_offset());
            self.set_state.set(state);
        }
    }

    pub fn set_filter_status(&self, status: Option<TaskStatus>) {
        let mut state = self.state.get();
        state.filters.status = status;
        state.pagination.page = 1;
        self.set_state.set(state);
    }

    pub fn set_filter_priority(&self, min: Option<u8>, max: Option<u8>) {
        let mut state = self.state.get();
        state.filters.priority_min = min;
        state.filters.priority_max = max;
        state.pagination.page = 1;
        self.set_state.set(state);
    }

    pub fn set_filter_team(&self, team_id: Option<u64>) {
        let mut state = self.state.get();
        state.filters.team_id = team_id;
        state.pagination.page = 1;
        self.set_state.set(state);
    }

    pub fn set_search_query(&self, query: Option<String>) {
        let mut state = self.state.get();
        state.filters.search_query = query;
        state.pagination.page = 1;
        self.set_state.set(state);
    }

    pub fn set_page(&self, page: u32) {
        let mut state = self.state.get();
        state.pagination.page = page;
        self.set_state.set(state);
    }

    pub fn set_page_size(&self, page_size: u32) {
        let mut state = self.state.get();
        state.pagination.page_size = page_size;
        state.pagination.page = 1;
        self.set_state.set(state);
    }

    pub fn clear_filters(&self) {
        let mut state = self.state.get();
        state.filters = TaskFilters::default();
        state.pagination.page = 1;
        self.set_state.set(state);
    }

    pub fn filtered_tasks(&self) -> Vec<Task> {
        let state = self.state.get();
        let filters = &state.filters;
        let mut tasks = state.tasks.clone();

        if let Some(status) = &filters.status {
            tasks.retain(|t| &t.task_status == status);
        }
        if let Some(min) = filters.priority_min {
            tasks.retain(|t| t.task_priority >= min);
        }
        if let Some(max) = filters.priority_max {
            tasks.retain(|t| t.task_priority <= max);
        }
        if let Some(team_id) = filters.team_id {
            tasks.retain(|t| t.task_team_id == Some(team_id));
        }
        if let Some(query) = &filters.search_query {
            let q = query.to_lowercase();
            tasks.retain(|t| {
                t.task_name.to_lowercase().contains(&q)
                    || t.task_description
                        .as_ref()
                        .is_some_and(|d| d.to_lowercase().contains(&q))
            });
        }

        tasks
    }
}

pub fn create_task_store() -> TaskStore {
    let (state, set_state) = signal(TaskListState::default());
    TaskStore { state, set_state }
}

fn chrono_offset() -> i64 {
    js_sys::Date::now() as i64
}
