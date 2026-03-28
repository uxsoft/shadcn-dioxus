use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn ButtonShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Button" }
        div {
            class: "flex flex-wrap gap-3 items-center",
            Button { "Default" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Destructive, "Destructive" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
            Button { variant: ButtonVariant::Link, "Link" }
        }
        div {
            class: "flex flex-wrap gap-3 items-center mt-4",
            Button { size: ButtonSize::Xs, "Extra Small" }
            Button { size: ButtonSize::Sm, "Small" }
            Button { size: ButtonSize::Default, "Default" }
            Button { size: ButtonSize::Lg, "Large" }
            Button { disabled: true, "Disabled" }
        }
    }
}
