use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn FormShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Form" }
        div {
            class: "max-w-sm space-y-4",
            FormField {
                Label { "Username" }
                Input { placeholder: "Enter your username" }
                FormDescription { "This is your public display name." }
            }
            FormField {
                Label { "Email" }
                Input { r#type: "email".to_string(), placeholder: "Enter your email" }
                FormMessage { message: "Email is required." }
            }
        }
    }
}
