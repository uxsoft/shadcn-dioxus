use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn HoverCardShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Hover Card" }
        HoverCard {
            HoverCardTrigger {
                Button { variant: ButtonVariant::Link, "@dioxuslabs" }
            }
            HoverCardContent {
                div {
                    class: "space-y-1",
                    h4 { class: "text-sm font-semibold", "@dioxuslabs" }
                    p { class: "text-sm text-muted-foreground",
                        "The Dioxus framework – fullstack, crossplatform, and blazingly fast."
                    }
                }
            }
        }
    }
}
