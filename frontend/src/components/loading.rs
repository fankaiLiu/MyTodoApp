use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum LoadingVariant {
    Spinner,
    Skeleton,
}

#[derive(Clone, PartialEq)]
pub enum SkeletonVariant {
    Text,
    Circle,
    Rect,
}

#[component]
pub fn Loading(
    #[prop(default = LoadingVariant::Spinner)] variant: LoadingVariant,
    #[prop(optional)] label: Option<String>,
    #[prop(default = 32)] size: u32,
) -> impl IntoView {
    let label_view = if let Some(text) = label {
        view! { <span class="loading-label">{text}</span> }.into_any()
    } else {
        ().into_any()
    };

    match variant {
        LoadingVariant::Spinner => view! {
            <div class="loading">
                <div class="spinner" style=format!("width: {}px; height: {}px;", size, size)>
                    <div class="spinner-circle"></div>
                </div>
                {label_view}
            </div>
        }
        .into_any(),
        LoadingVariant::Skeleton => view! {
            <div class="loading">
                <div class="skeleton">
                    <Skeleton variant=SkeletonVariant::Text width="100%".to_string() />
                    <Skeleton variant=SkeletonVariant::Text width="80%".to_string() />
                    <Skeleton variant=SkeletonVariant::Text width="60%".to_string() />
                </div>
                {label_view}
            </div>
        }
        .into_any(),
    }
}

#[component]
pub fn Skeleton(
    #[prop(default = SkeletonVariant::Text)] variant: SkeletonVariant,
    #[prop(default = "100%".to_string())] width: String,
    #[prop(default = 16)] height: u32,
) -> impl IntoView {
    let class = match variant {
        SkeletonVariant::Text => "skeleton-line",
        SkeletonVariant::Circle => "skeleton-circle",
        SkeletonVariant::Rect => "skeleton-rect",
    };

    view! {
        <div
            class=("skeleton-item", true)
            class=(class, true)
            style=format!("width: {}; height: {}px;", width, height)
        >
        </div>
    }
}
