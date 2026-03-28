use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn AspectRatioShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Aspect Ratio" }
        div {
            class: "max-w-[300px]",
            AspectRatio {
                ratio: 16.0 / 9.0,
                div {
                    class: "flex items-center justify-center w-full h-full bg-muted rounded-md text-sm text-muted-foreground",
                    "16:9"
                }
            }
        }
    }
}
