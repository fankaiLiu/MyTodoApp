use crate::store::{get_local_storage_item, set_local_storage_item};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

const THEME_KEY: &str = "todo_theme";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeStore {
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

impl ThemeStore {
    pub fn toggle(&self) {
        let current = self.theme.get();
        let new_theme = match current {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
        self.set_theme.set(new_theme);
    }

    pub fn apply_to_dom(&self) {
        let theme = self.theme.get();
        if let Some(html) = document().document_element() {
            if let Some(el) = html.dyn_ref::<web_sys::HtmlElement>() {
                let _ = el.set_attribute("data-theme", theme.as_str());
            }
        }
    }
}

pub fn create_theme_store() -> ThemeStore {
    let stored = get_local_storage_item(THEME_KEY).unwrap_or_else(|| "dark".to_string());
    let initial = Theme::from_str(&stored);

    let (theme, set_theme) = signal(initial);

    let _ = Effect::new(move || {
        let t = theme.get();
        set_local_storage_item(THEME_KEY, t.as_str());
        if let Some(html) = document().document_element() {
            if let Some(el) = html.dyn_ref::<web_sys::HtmlElement>() {
                let _ = el.set_attribute("data-theme", t.as_str());
            }
        }
    });

    ThemeStore { theme, set_theme }
}
