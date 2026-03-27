use dioxus::prelude::*;

use crate::components::ToastState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeState {
    pub dark: bool,
}

impl ThemeState {
    pub fn toggle_dark(&mut self) {
        self.dark = !self.dark;
    }
}

/// Provides theme context (dark mode) to the component tree.
/// Wrap your app's root in this component.
///
/// The `accent_color` prop accepts an oklch color string (e.g. "oklch(0.6 0.25 250)")
/// that overrides `--primary` and related variables at runtime.
#[component]
pub fn ThemeProvider(
    #[props(default)] accent_color: Option<String>,
    children: Element,
) -> Element {
    let theme = use_context_provider(|| Signal::new(ThemeState { dark: false }));
    let toasts = use_context_provider(|| Signal::new(ToastState::new()));

    let dark_class = if theme.read().dark { "dark" } else { "" };

    let accent_style = match &accent_color {
        Some(color) => format!(
            "--primary: {}; --ring: {};",
            color, color,
        ),
        None => String::new(),
    };

    rsx! {
        div {
            class: "{dark_class}",
            style: "{accent_style}",
            {children}
        }
    }
}
