use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn CardShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Card" }
        div {
            class: "max-w-sm",
            Card {
                CardHeader {
                    CardTitle { "Create project" }
                    CardDescription { "Deploy your project in one-click." }
                }
                CardContent {
                    p { class: "text-sm text-muted-foreground", "Project configuration goes here." }
                }
                CardFooter {
                    class: "justify-between",
                    Button { variant: ButtonVariant::Outline, "Cancel" }
                    Button { "Deploy" }
                }
            }
        }
    }
}
