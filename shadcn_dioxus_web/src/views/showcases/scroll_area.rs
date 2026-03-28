use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn ScrollAreaShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Scroll Area" }
        ScrollArea {
            class: "h-[200px] w-[250px] rounded-md border p-4".to_string(),
            for i in 0..20 {
                div {
                    class: "py-1 text-sm",
                    "Item {i}"
                }
                if i < 19 {
                    Separator {}
                }
            }
        }
    }
}
