use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
        }
    }
}

#[component]
pub fn Input(
    #[prop(default = InputType::Text)] input_type: InputType,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] error: Option<String>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional)] on_input: Option<Callback<(String,)>>,
    #[prop(optional)] on_change: Option<Callback<(String,)>>,
    #[prop(optional)] on_blur: Option<Callback<(ev::FocusEvent,)>>,
) -> impl IntoView {
    let input_id = id.unwrap_or_else(|| {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("input-{}", n)
    });
    let has_error = error.is_some();
    let error_text = error.unwrap_or_default();

    let on_input_handler = {
        let on_input = on_input;
        move |ev: ev::Event| {
            let value = event_target_value(&ev);
            if let Some(cb) = on_input {
                cb.run((value,));
            }
        }
    };

    let on_change_handler = {
        let on_change = on_change;
        move |ev: ev::Event| {
            let value = event_target_value(&ev);
            if let Some(cb) = on_change {
                cb.run((value,));
            }
        }
    };

    let on_blur_handler = {
        let on_blur = on_blur;
        move |ev: ev::FocusEvent| {
            if let Some(cb) = on_blur {
                cb.run((ev,));
            }
        }
    };

    let id_clone = input_id.clone();
    let required_star = if required {
        view! { <span class="input-required">" *"</span> }.into_any()
    } else {
        ().into_any()
    };
    let label_view = label.map(|text| {
        let id_clone = id_clone.clone();
        view! {
            <label for=id_clone class="input-label">
                {text}
                {required_star}
            </label>
        }
    });

    let error_view = if has_error {
        view! { <span class="input-error-text">{error_text}</span> }.into_any()
    } else {
        ().into_any()
    };

    view! {
        <div class="input-wrapper">
            {label_view}
            <input
                id=input_id
                type=input_type.as_str()
                class=("input-field", true)
                class=("input-error", has_error)
                class=("input-disabled", disabled)
                placeholder=placeholder
                value=value
                disabled=disabled
                required=required
                on:input=on_input_handler
                on:change=on_change_handler
                on:blur=on_blur_handler
            />
            {error_view}
        </div>
    }
}
