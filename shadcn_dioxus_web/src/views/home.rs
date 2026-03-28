use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 py-8 max-w-6xl",

            // Hero
            div {
                class: "mb-12 text-center py-20",
                h1 {
                    class: "text-4xl font-bold tracking-tight mb-4",
                    "shadcn/dioxus"
                }
                p {
                    class: "text-xl text-muted-foreground max-w-2xl mx-auto mb-8",
                    "A port of shadcn/ui components for Dioxus. Beautiful, accessible, and customizable."
                }
                Link {
                    to: Route::ShowcasePage { name: "button".to_string() },
                    class: "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-6 py-2",
                    "Browse Components"
                }
            }
        }
    }
}
