use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn AvatarShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Avatar" }
        div {
            class: "flex gap-4 items-center",
            Avatar {
                AvatarFallback { "JD" }
            }
            Avatar {
                size: AvatarSize::Sm,
                AvatarFallback { "SM" }
            }
            Avatar {
                size: AvatarSize::Lg,
                AvatarFallback { "LG" }
            }
        }
    }
}
