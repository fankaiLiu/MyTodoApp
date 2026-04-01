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
pub fn Button(
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] full_width: bool,
    #[prop(optional)] on_click: Option<Callback<(ev::MouseEvent,)>>,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Ghost => "btn-ghost",
    };

    let size_class = match size {
        ButtonSize::Sm => "btn-sm",
        ButtonSize::Md => "btn-md",
        ButtonSize::Lg => "btn-lg",
    };

    let on_click_handler = move |ev: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run((ev,));
        }
    };

    view! {
        <button
            class=("btn", true)
            class=(variant_class, true)
            class=(size_class, true)
            class=("btn-full-width", full_width)
            class=("btn-disabled", disabled)
            disabled=disabled
            on:click=on_click_handler
        >
            {children()}
        </button>
    }
}
