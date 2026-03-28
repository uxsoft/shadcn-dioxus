use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn AlertShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Alert" }
        div {
            class: "space-y-4 max-w-lg",
            Alert {
                AlertTitle { "Heads up!" }
                AlertDescription { "You can add components to your app using the CLI." }
            }
            Alert {
                variant: AlertVariant::Destructive,
                AlertTitle { "Error" }
                AlertDescription { "Your session has expired. Please log in again." }
            }
        }
    }
}
