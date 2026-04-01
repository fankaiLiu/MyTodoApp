use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Modal(
    #[prop(optional)] title: Option<String>,
    #[prop(default = MaybeSignal::Static(true))] open: MaybeSignal<bool>,
    #[prop(default = true)] close_on_overlay: bool,
    #[prop(default = true)] show_close_button: bool,
    #[prop(optional)] on_close: Option<Callback<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let close_on_overlay_click = move |ev: ev::MouseEvent| {
        if close_on_overlay {
            if let Some(target_el) = ev.target() {
                if let Some(el) = target_el.dyn_ref::<web_sys::Element>() {
                    if el.class_list().contains("modal-overlay") {
                        if let Some(cb) = on_close {
                            cb.run(());
                        }
                    }
                }
            }
        }
    };

    let close_handler = move || {
        if let Some(cb) = on_close {
            cb.run(());
        }
    };

    view! {
        {move || {
            if open.get() {
                view! {
                    <div class="modal-overlay" on:click=close_on_overlay_click>
                        <div class="modal-container" role="dialog" aria-modal="true">
                            <div class="modal-header">
                                {if let Some(text) = title.clone() {
                                    view! { <h2 class="modal-title">{text}</h2> }.into_any()
                                } else {
                                    ().into_any()
                                }}
                                {if show_close_button {
                                    view! {
                                        <button class="modal-close" on:click=move |_| close_handler() aria-label="Close">
                                            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                                <path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                            </svg>
                                        </button>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </div>
                            <div class="modal-body">
                                {children()}
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }
        }}
    }
}
