use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn SelectShowcase() -> Element {
    let mut select_val = use_signal(|| String::new());

    rsx! {
        SectionTitle { title: "Select" }
        div {
            class: "max-w-[200px]",
            Select {
                value: select_val(),
                onchange: move |v| select_val.set(v),
                SelectTrigger {
                    placeholder: "Select a fruit".to_string(),
                    SelectValue { placeholder: "Select a fruit" }
                }
                SelectContent {
                    SelectGroup {
                        SelectLabel { "Fruits" }
                        SelectItem { value: "apple", "Apple" }
                        SelectItem { value: "banana", "Banana" }
                        SelectItem { value: "blueberry", "Blueberry" }
                        SelectItem { value: "grapes", "Grapes" }
                        SelectItem { value: "pineapple", "Pineapple" }
                    }
                }
            }
        }
    }
}
