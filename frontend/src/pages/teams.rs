use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::card::Card;
use crate::store::use_team_store;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn TeamsPage() -> impl IntoView {
    let team_store = use_team_store();
    let navigate = use_navigate();

    let nav_back = {
        let n = navigate.clone();
        move |_| n("/", Default::default())
    };

    view! {
        <div class="page">
            <header class="page-header">
                <div>
                    <button class="back-btn" on:click=nav_back>"← Back"</button>
                    <h1 class="page-title">"Teams"</h1>
                </div>
                <Button variant=ButtonVariant::Primary size=ButtonSize::Sm>"New Team"</Button>
            </header>

            {move || {
                let state = team_store.state.get();
                if state.teams.is_empty() {
                    view! {
                        <Card title="No Teams".to_string() subtitle="Create or join a team to collaborate".to_string()>
                            <p class="empty-text">"You haven't joined any teams yet."</p>
                        </Card>
                    }.into_any()
                } else {
                    let ts = team_store.clone();
                    let cards: Vec<_> = state.teams.into_iter().map(|team| {
                        let member_count = team.team_members.len();
                        let desc = team.team_settings.team_description.clone();
                        let team_id = team.team_id;
                        let ts = ts.clone();
                        view! {
                            <Card
                                title=team.team_name.clone()
                                subtitle=format!("{} members", member_count)
                                interactive=true
                                on_click=Callback::from(move |_| {
                                    ts.set_active_team(Some(team_id));
                                })
                            >
                                {if let Some(d) = desc.clone() {
                                    view! { <p class="team-desc">{d.clone()}</p> }.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </Card>
                        }
                    }).collect();
                    view! { <div class="team-list">{cards}</div> }.into_any()
                }
            }}
        </div>
    }
}
