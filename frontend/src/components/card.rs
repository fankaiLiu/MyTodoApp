use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Card(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] subtitle: Option<String>,
    #[prop(optional)] on_click: Option<Callback<(ev::MouseEvent,)>>,
    #[prop(default = false)] interactive: bool,
    #[prop(default = false)] elevated: bool,
    children: Children,
) -> impl IntoView {
    let on_click_handler = move |ev: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run((ev,));
        }
    };

    view! {
        <div
            class=("card", true)
            class=("card-interactive", interactive)
            class=("card-elevated", elevated)
            on:click=on_click_handler
        >
            {if title.is_some() || subtitle.is_some() {
                view! {
                    <div class="card-header">
                        {if let Some(text) = title {
                            view! { <h3 class="card-title">{text}</h3> }.into_any()
                        } else {
                            ().into_any()
                        }}
                        {if let Some(text) = subtitle {
                            view! { <p class="card-subtitle">{text}</p> }.into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}
            <div class="card-body">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn CardFooter(children: Children) -> impl IntoView {
    view! {
        <div class="card-footer">
            {children()}
        </div>
    }
}
