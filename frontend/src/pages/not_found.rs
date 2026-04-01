use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <div class="not-found">
            <h1 class="not-found-code">"404"</h1>
            <p class="not-found-message">"The page you're looking for doesn't exist."</p>
            <button
                class="not-found-btn"
                on:click=move |_| navigate("/", Default::default())
            >
                "Go Home"
            </button>
        </div>
    }
}
