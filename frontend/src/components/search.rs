use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn SearchInput(
    #[prop(default = "Search...".to_string())] placeholder: String,
    #[prop(optional)] on_search: Option<Callback<(String,)>>,
    #[prop(default = false)] instant: bool,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());

    let handle_input = {
        let cb = on_search.clone();
        move |ev: ev::Event| {
            let value = event_target_value(&ev);
            set_query.set(value.clone());
            if instant {
                if let Some(callback) = cb.as_ref() {
                    callback.run((value,));
                }
            }
        }
    };

    let handle_keydown = {
        let q = query.clone();
        let cb = on_search.clone();
        move |ev: ev::KeyboardEvent| {
            if ev.key() == "Enter" {
                if let Some(callback) = cb.as_ref() {
                    callback.run((q.get(),));
                }
            }
        }
    };

    let handle_clear = {
        let set_q = set_query.clone();
        let cb = on_search.clone();
        move |_: ev::MouseEvent| {
            set_q.set(String::new());
            if let Some(callback) = cb.as_ref() {
                callback.run((String::new(),));
            }
        }
    };

    view! {
        <div class="search-input-wrapper">
            <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="11" cy="11" r="8"/>
                <path d="m21 21-4.35-4.35"/>
            </svg>
            <input
                type="text"
                class="search-input"
                placeholder=placeholder
                value=query.get()
                on:input=handle_input
                on:keydown=handle_keydown
            />
            {move || {
                if !query.get().is_empty() {
                    view! {
                        <button class="search-clear" on:click=handle_clear>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M18 6 6 18M6 6l12 12"/>
                            </svg>
                        </button>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn FilterSelect(
    #[prop(default = "Filter".to_string())] label: String,
    options: Vec<(String, String)>,
    #[prop(optional)] on_change: Option<Callback<(String,)>>,
) -> impl IntoView {
    let (selected, set_selected) = signal(String::new());

    let handle_change = {
        let cb = on_change.clone();
        move |ev: ev::Event| {
            let value = event_target_value(&ev);
            set_selected.set(value.clone());
            if let Some(callback) = cb.as_ref() {
                callback.run((value,));
            }
        }
    };

    view! {
        <select class="filter-select" on:change=handle_change>
            <option value="" disabled selected={selected.get().is_empty()}>{label}</option>
            {options.into_iter().map(|(value, label)| {
                let val = value.clone();
                view! { <option value={val} selected={selected.get() == value}>{label}</option> }
            }).collect::<Vec<_>>()}
        </select>
    }
}

#[component]
pub fn Pagination(
    current_page: u32,
    total_pages: u32,
    #[prop(optional)] on_page_change: Option<Callback<(u32,)>>,
) -> impl IntoView {
    let can_prev = move || current_page > 1;
    let can_next = move || current_page < total_pages;

    let go_prev = {
        let cb = on_page_change.clone();
        move |_: ev::MouseEvent| {
            if let Some(callback) = cb.as_ref() {
                callback.run((current_page - 1,));
            }
        }
    };

    let go_next = {
        let cb = on_page_change.clone();
        move |_: ev::MouseEvent| {
            if let Some(callback) = cb.as_ref() {
                callback.run((current_page + 1,));
            }
        }
    };

    view! {
        <div class="pagination">
            <button
                class="pagination-btn"
                disabled=!can_prev()
                on:click=go_prev
            >
                "Previous"
            </button>
            <span class="pagination-info">
                {current_page} " / " {total_pages}
            </span>
            <button
                class="pagination-btn"
                disabled=!can_next()
                on:click=go_next
            >
                "Next"
            </button>
        </div>
    }
}
