use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn SwitchShowcase() -> Element {
    let mut switch_checked = use_signal(|| true);

    rsx! {
        SectionTitle { title: "Switch" }
        div {
            class: "flex items-center gap-4",
            Switch {
                checked: switch_checked(),
                onchange: move |v| switch_checked.set(v),
            }
            span { class: "text-sm", "Enabled: {switch_checked}" }
        }
    }
}
