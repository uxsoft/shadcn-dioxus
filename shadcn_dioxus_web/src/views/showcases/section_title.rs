use dioxus::prelude::*;
use crate::components::*;

#[component]
pub fn SectionTitle(title: String) -> Element {
    rsx! {
        div {
            class: "mt-12 mb-4",
            h2 {
                class: "text-2xl font-semibold tracking-tight",
                "{title}"
            }
            Separator { class: "mt-2".to_string() }
        }
    }
}
