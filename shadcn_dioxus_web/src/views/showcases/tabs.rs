use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn TabsShowcase() -> Element {
    let mut tab_val = use_signal(|| "account".to_string());

    rsx! {
        SectionTitle { title: "Tabs" }
        div {
            class: "max-w-md",
            Tabs {
                value: tab_val(),
                onchange: move |v| tab_val.set(v),
                TabsList {
                    TabsTrigger { value: "account", "Account" }
                    TabsTrigger { value: "password", "Password" }
                }
                TabsContent {
                    value: "account",
                    Card {
                        CardHeader {
                            CardTitle { "Account" }
                            CardDescription { "Make changes to your account here." }
                        }
                        CardContent {
                            p { class: "text-sm", "Account content goes here." }
                        }
                    }
                }
                TabsContent {
                    value: "password",
                    Card {
                        CardHeader {
                            CardTitle { "Password" }
                            CardDescription { "Change your password here." }
                        }
                        CardContent {
                            p { class: "text-sm", "Password content goes here." }
                        }
                    }
                }
            }
        }
    }
}
