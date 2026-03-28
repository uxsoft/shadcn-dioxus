use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn TooltipShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Tooltip" }
        div {
            class: "flex gap-4",
            Tooltip {
                TooltipTrigger {
                    Button { variant: ButtonVariant::Outline, "Hover me" }
                }
                TooltipContent {
                    "Add to library"
                }
            }
        }
    }
}
