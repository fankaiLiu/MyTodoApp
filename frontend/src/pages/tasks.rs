use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::card::Card;
use crate::components::loading::{Loading, LoadingVariant};
use crate::components::search::{Pagination, SearchInput};
use crate::components::task_card::{TaskCard, TaskCardSkeleton};
use crate::store::task_store::{Task, TaskStatus};
use crate::store::use_task_store;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn TasksPage() -> impl IntoView {
    let task_store = use_task_store();
    let navigate = use_navigate();

    let nav_back = {
        let n = navigate.clone();
        move |_| n("/", Default::default())
    };

    let ts = task_store.clone();
    let filter_all = Callback::from(move |_| {
        ts.set_filter_status(None);
    });
    let ts = task_store.clone();
    let filter_active = Callback::from(move |_| {
        ts.set_filter_status(Some(TaskStatus::Active));
    });
    let ts = task_store.clone();
    let filter_completed = Callback::from(move |_| {
        ts.set_filter_status(Some(TaskStatus::Completed));
    });
    let ts = task_store.clone();
    let filter_paused = Callback::from(move |_| {
        ts.set_filter_status(Some(TaskStatus::Paused));
    });

    let handle_search = {
        let ts = task_store.clone();
        Callback::from(move |query: String| {
            ts.set_search_query(if query.is_empty() { None } else { Some(query) });
        })
    };

    let current_filter = {
        let store = task_store.clone();
        move || store.state.get().filters.status.clone()
    };

    let current_page = {
        let store = task_store.clone();
        move || store.state.get().pagination.page
    };
    let total_pages = {
        let store = task_store.clone();
        move || {
            let state = store.state.get();
            let total = state.pagination.total as f32;
            let page_size = state.pagination.page_size as f32;
            if total == 0.0 {
                1
            } else {
                (total / page_size).ceil() as u32
            }
        }
    };

    let handle_page_change = {
        let ts = task_store.clone();
        Callback::from(move |page: u32| {
            ts.set_page(page);
        })
    };

    view! {
        <div class="page">
            <header class="page-header">
                <div>
                    <button class="back-btn" on:click=nav_back>"← Back"</button>
                    <h1 class="page-title">"Tasks"</h1>
                </div>
                <Button variant=ButtonVariant::Primary size=ButtonSize::Sm>"New Task"</Button>
            </header>

            <div class="task-toolbar">
                <SearchInput
                    placeholder="Search tasks...".to_string()
                    instant=true
                    on_search=handle_search
                />
            </div>

            <div class="filter-bar">
                <Button
                    variant=ButtonVariant::Secondary
                    size=ButtonSize::Sm
                    on_click=filter_all
                >
                    "All"
                </Button>
                <Button
                    variant=ButtonVariant::Secondary
                    size=ButtonSize::Sm
                    on_click=filter_active
                >
                    "Active"
                </Button>
                <Button
                    variant=ButtonVariant::Secondary
                    size=ButtonSize::Sm
                    on_click=filter_completed
                >
                    "Completed"
                </Button>
                <Button
                    variant=ButtonVariant::Secondary
                    size=ButtonSize::Sm
                    on_click=filter_paused
                >
                    "Paused"
                </Button>
            </div>

            <div class="tasks-content">
                {{
                    let store = task_store.clone();
                    move || {
                        let state = store.state.get();
                        if state.is_loading {
                            view! {
                                <div class="task-list">
                                    <TaskCardSkeleton />
                                    <TaskCardSkeleton />
                                    <TaskCardSkeleton />
                                </div>
                            }.into_any()
                        } else {
                            let filtered = store.filtered_tasks();
                            if filtered.is_empty() {
                                view! {
                                    <Card title="No Tasks".to_string() subtitle="Create your first task to get started".to_string()>
                                        <p class="empty-text">"No tasks found matching your filters."</p>
                                    </Card>
                                }.into_any()
                            } else {
                                let cards: Vec<_> = filtered.into_iter().map(|task| {
                                    view! {
                                        <TaskCard task=task interactive=true />
                                    }
                                }).collect();
                                view! { <div class="task-grid">{cards}</div> }.into_any()
                            }
                        }
                    }
                }}
            </div>

            {{
                let store = task_store.clone();
                move || {
                    let total = store.state.get().pagination.total;
                    if total > 0 {
                        view! {
                            <Pagination
                                current_page=current_page()
                                total_pages=total_pages()
                                on_page_change=handle_page_change
                            />
                        }.into_any()
                    } else {
                        ().into_any()
                    }
                }
            }}
        </div>
    }
}
