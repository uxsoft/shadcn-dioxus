use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn KbdShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Kbd" }
        div {
            class: "flex gap-2 items-center",
            Kbd { "⌘" }
            Kbd { "K" }
            span { class: "text-sm text-muted-foreground", "to open command palette" }
        }
    }
}
