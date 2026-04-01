use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::card::{Card, CardFooter};
use crate::store::task_store::TaskStatus;
use crate::store::{use_task_store, use_team_store, use_theme_store, use_user_store};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let user_store = use_user_store();
    let theme_store = use_theme_store();
    let task_store = use_task_store();
    let team_store = use_team_store();
    let navigate = use_navigate();

    let profile = move || user_store.profile();

    let nav_tasks = {
        let nav = navigate.clone();
        Callback::from(move |_| nav("/tasks", Default::default()))
    };
    let nav_teams = {
        let nav = navigate.clone();
        Callback::from(move |_| nav("/teams", Default::default()))
    };
    let nav_settings = {
        let nav = navigate.clone();
        Callback::from(move |_| nav("/settings", Default::default()))
    };

    view! {
        <div class="dashboard">
            <header class="dashboard-header">
                <div>
                    <h1 class="dashboard-title">"Dashboard"</h1>
                    {move || {
                        if let Some(p) = profile() {
                            let name = p.username.clone();
                            view! { <p class="dashboard-greeting">{"Welcome back, "}{name}</p> }.into_any()
                        } else {
                            view! { <p class="dashboard-greeting">{"Welcome"}</p> }.into_any()
                        }
                    }}
                </div>
                <div class="dashboard-actions">
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm on_click=nav_tasks>"Tasks"</Button>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm on_click=nav_teams>"Teams"</Button>
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on_click=nav_settings>"Settings"</Button>
                </div>
            </header>

            <div class="dashboard-grid">
                <Card title="Tasks".to_string() subtitle="Your task overview".to_string()>
                    <div class="stat-row">
                        <div class="stat">
                            <span class="stat-number">{move || task_store.state.get().tasks.len()}</span>
                            <span class="stat-label">"Total"</span>
                        </div>
                        <div class="stat">
                            <span class="stat-number">{move || task_store.state.get().tasks.iter().filter(|t| matches!(t.task_status, TaskStatus::Active)).count()}</span>
                            <span class="stat-label">"Active"</span>
                        </div>
                    </div>
                    <CardFooter>
                        <Button variant=ButtonVariant::Primary size=ButtonSize::Sm on_click=nav_tasks>"View All"</Button>
                    </CardFooter>
                </Card>

                <Card title="Teams".to_string() subtitle="Your team memberships".to_string()>
                    <div class="stat-row">
                        <div class="stat">
                            <span class="stat-number">{move || team_store.state.get().teams.len()}</span>
                            <span class="stat-label">"Teams"</span>
                        </div>
                    </div>
                    <CardFooter>
                        <Button variant=ButtonVariant::Primary size=ButtonSize::Sm on_click=nav_teams>"View All"</Button>
                    </CardFooter>
                </Card>

                <Card title="Theme".to_string() subtitle="Appearance settings".to_string()>
                    <p class="theme-info">
                        "Current: "
                        {move || match theme_store.theme.get() {
                            crate::store::theme_store::Theme::Light => "Light",
                            crate::store::theme_store::Theme::Dark => "Dark",
                        }}
                    </p>
                    <CardFooter>
                        <Button
                            variant=ButtonVariant::Secondary
                            size=ButtonSize::Sm
                            on_click=Callback::from(move |_| { theme_store.toggle(); })
                        >"Toggle Theme"</Button>
                    </CardFooter>
                </Card>
            </div>
        </div>
    }
}
