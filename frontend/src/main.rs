use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod components;
mod api;
mod store;
mod pages;

use pages::dashboard::DashboardPage;
use pages::login::LoginPage;
use pages::not_found::NotFoundPage;
use pages::protected_route::ProtectedRoute;
use pages::register::RegisterPage;
use pages::settings::SettingsPage;
use pages::tasks::TasksPage;
use pages::teams::TeamsPage;
use store::{create_stores, provide_stores, use_theme_store};

#[component]
fn App() -> impl IntoView {
    let stores = create_stores();
    provide_stores(stores);

    let theme_store = use_theme_store();

    view! {
        <Router>
            <div class="app-container">
                <AppHeader theme_store=theme_store />
                <main class="app-main">
                    <Routes fallback=|| view! { <NotFoundPage /> }>
                        <Route path=path!("/") view=LoginPage />
                        <Route path=path!("/login") view=LoginPage />
                        <Route path=path!("/register") view=RegisterPage />
                        <Route
                            path=path!("/dashboard")
                            view=|| view! {
                                <ProtectedRoute>
                                    <DashboardPage />
                                </ProtectedRoute>
                            }
                        />
                        <Route
                            path=path!("/tasks")
                            view=|| view! {
                                <ProtectedRoute>
                                    <TasksPage />
                                </ProtectedRoute>
                            }
                        />
                        <Route
                            path=path!("/teams")
                            view=|| view! {
                                <ProtectedRoute>
                                    <TeamsPage />
                                </ProtectedRoute>
                            }
                        />
                        <Route
                            path=path!("/settings")
                            view=|| view! {
                                <ProtectedRoute>
                                    <SettingsPage />
                                </ProtectedRoute>
                            }
                        />
                        <Route path=path!("/*any") view=NotFoundPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn AppHeader(theme_store: store::theme_store::ThemeStore) -> impl IntoView {
    let toggle_theme = move |_| {
        theme_store.toggle();
    };

    view! {
        <header class="app-header">
            <a href="/dashboard" class="app-logo">"todoManager"</a>
            <div class="header-actions">
                <button class="theme-toggle" on:click=toggle_theme>
                    {move || match theme_store.theme.get() {
                        store::theme_store::Theme::Light => "☀️",
                        store::theme_store::Theme::Dark => "🌙",
                    }}
                </button>
            </div>
        </header>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> })
}
