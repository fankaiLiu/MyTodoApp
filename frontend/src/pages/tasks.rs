use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::card::Card;
use crate::components::loading::{Loading, LoadingVariant};
use crate::store::task_store::TaskStatus;
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

    view! {
        <div class="page">
            <header class="page-header">
                <div>
                    <button class="back-btn" on:click=nav_back>"← Back"</button>
                    <h1 class="page-title">"Tasks"</h1>
                </div>
                <Button variant=ButtonVariant::Primary size=ButtonSize::Sm>"New Task"</Button>
            </header>

            <div class="filter-bar">
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm on_click=filter_all>"All"</Button>
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm on_click=filter_active>"Active"</Button>
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm on_click=filter_completed>"Completed"</Button>
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm on_click=filter_paused>"Paused"</Button>
            </div>

            {move || {
                let state = task_store.state.get();
                if state.is_loading {
                    view! { <Loading variant=LoadingVariant::Spinner label="Loading tasks...".to_string() /> }.into_any()
                } else {
                    let filtered = task_store.filtered_tasks();
                    if filtered.is_empty() {
                        view! {
                            <Card title="No Tasks".to_string() subtitle="Create your first task to get started".to_string()>
                                <p class="empty-text">"No tasks found matching your filters."</p>
                            </Card>
                        }.into_any()
                    } else {
                        let cards: Vec<_> = filtered.into_iter().map(|task| {
                            let status_label = match &task.task_status {
                                TaskStatus::Active => "Active",
                                TaskStatus::Completed => "Completed",
                                TaskStatus::Paused => "Paused",
                            };
                            let desc = task.task_description.clone();
                            let deadline = task.task_deadline;
                            let priority = task.task_priority;
                            view! {
                                <Card
                                    title=task.task_name.clone()
                                    subtitle=status_label.to_string()
                                    interactive=true
                                >
                                    {if let Some(d) = desc {
                                        view! { <p class="task-desc">{d.clone()}</p> }.into_any()
                                    } else {
                                        ().into_any()
                                    }}
                                    <div class="task-meta">
                                        <span class="task-priority">{"Priority: "}{priority}</span>
                                        {if let Some(dl) = deadline {
                                            view! { <span class="task-deadline">{"Due: "}{dl}</span> }.into_any()
                                        } else {
                                            ().into_any()
                                        }}
                                    </div>
                                </Card>
                            }
                        }).collect();
                        view! { <div class="task-list">{cards}</div> }.into_any()
                    }
                }
            }}
        </div>
    }
}
