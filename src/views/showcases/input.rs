use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn InputShowcase() -> Element {
    let mut input_val = use_signal(|| String::new());

    rsx! {
        SectionTitle { title: "Input" }
        div {
            class: "grid gap-4 max-w-sm",
            Input {
                value: input_val(),
                placeholder: "Type something...",
                oninput: move |e: FormEvent| *input_val.write() = e.value(),
            }
            Input {
                disabled: true,
                placeholder: "Disabled input",
            }
        }
    }
}
