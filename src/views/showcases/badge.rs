use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn BadgeShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Badge" }
        div {
            class: "flex flex-wrap gap-3 items-center",
            Badge { "Default" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
        }
    }
}
