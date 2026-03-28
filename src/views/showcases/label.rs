use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn LabelShowcase() -> Element {
    let mut checkbox_checked = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Label" }
        div {
            class: "flex items-center gap-2",
            Checkbox {
                checked: checkbox_checked(),
                id: Some("terms".to_string()),
                onchange: move |v| checkbox_checked.set(v),
            }
            Label {
                r#for: "terms".to_string(),
                "Accept terms and conditions"
            }
        }
    }
}
