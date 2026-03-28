use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn SkeletonShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Skeleton" }
        div {
            class: "flex items-center gap-4",
            Skeleton { class: "h-12 w-12 rounded-full".to_string() }
            div {
                class: "space-y-2",
                Skeleton { class: "h-4 w-[250px]".to_string() }
                Skeleton { class: "h-4 w-[200px]".to_string() }
            }
        }
    }
}
