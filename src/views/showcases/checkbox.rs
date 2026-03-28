use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn CheckboxShowcase() -> Element {
    let mut checkbox_checked = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Checkbox" }
        div {
            class: "flex items-center gap-4",
            Checkbox {
                checked: checkbox_checked(),
                onchange: move |v| checkbox_checked.set(v),
            }
            span { class: "text-sm", "Checked: {checkbox_checked}" }
        }
    }
}
