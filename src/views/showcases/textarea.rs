use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn TextareaShowcase() -> Element {
    let mut textarea_val = use_signal(|| String::new());

    rsx! {
        SectionTitle { title: "Textarea" }
        div {
            class: "max-w-sm",
            Textarea {
                value: textarea_val(),
                placeholder: "Write your message...",
                oninput: move |e: FormEvent| *textarea_val.write() = e.value(),
            }
        }
    }
}
