use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn InputGroupShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Input Group" }
        div {
            class: "max-w-sm space-y-4",
            InputGroup {
                InputGroupAddon { "https://" }
                InputGroupInput { placeholder: "www.example.com" }
            }
            InputGroup {
                InputGroupInput { placeholder: "0.00" }
                InputGroupAddon { "USD" }
            }
        }
    }
}
