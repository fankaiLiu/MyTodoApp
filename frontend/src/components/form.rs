use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Form(
    #[prop(optional)] on_submit: Option<Callback<(ev::SubmitEvent,)>>,
    #[prop(default = false)] compact: bool,
    children: Children,
) -> impl IntoView {
    let on_submit_handler = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(cb) = on_submit {
            cb.run((ev,));
        }
    };

    view! {
        <form
            class=("form", true)
            class=("form-compact", compact)
            on:submit=on_submit_handler
        >
            {children()}
        </form>
    }
}

#[component]
pub fn FormGroup(
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] error: Option<String>,
    #[prop(default = false)] required: bool,
    children: Children,
) -> impl IntoView {
    let has_error = error.is_some();
    let error_text = error.unwrap_or_default();

    let required_star = if required {
        view! { <span class="form-required">" *"</span> }.into_any()
    } else {
        ().into_any()
    };

    let error_view = if has_error {
        view! { <span class="form-error">{error_text}</span> }.into_any()
    } else {
        ().into_any()
    };

    view! {
        <div class=("form-group", true)>
            {if let Some(text) = label {
                view! {
                    <label class="form-label">
                        {text}
                        {required_star}
                    </label>
                }.into_any()
            } else {
                ().into_any()
            }}
            {children()}
            {error_view}
        </div>
    }
}

#[component]
pub fn FormActions(children: Children) -> impl IntoView {
    view! {
        <div class="form-actions">
            {children()}
        </div>
    }
}
