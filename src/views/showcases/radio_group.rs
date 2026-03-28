use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn RadioGroupShowcase() -> Element {
    let mut radio_val = use_signal(|| "option-1".to_string());

    rsx! {
        SectionTitle { title: "Radio Group" }
        div {
            class: "max-w-sm",
            RadioGroup {
                value: radio_val(),
                onchange: move |v| radio_val.set(v),
                div {
                    class: "flex items-center gap-2",
                    RadioGroupItem { value: "option-1", id: Some("r1".to_string()) }
                    Label { r#for: "r1".to_string(), "Default" }
                }
                div {
                    class: "flex items-center gap-2",
                    RadioGroupItem { value: "option-2", id: Some("r2".to_string()) }
                    Label { r#for: "r2".to_string(), "Comfortable" }
                }
                div {
                    class: "flex items-center gap-2",
                    RadioGroupItem { value: "option-3", id: Some("r3".to_string()) }
                    Label { r#for: "r3".to_string(), "Compact" }
                }
            }
        }
    }
}
