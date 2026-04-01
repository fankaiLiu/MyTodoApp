use crate::store::use_user_store;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn ProtectedRoute(children: ChildrenFn) -> impl IntoView {
    let user_store = use_user_store();
    let navigate = use_navigate();

    let is_authed = move || user_store.is_authenticated();

    let is_authed_clone = is_authed.clone();
    Effect::new(move || {
        if !is_authed_clone() {
            navigate("/login", Default::default());
        }
    });

    view! {
        <Show when=is_authed>
            {children()}
        </Show>
    }
}
