use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn ToastShowcase() -> Element {
    let mut toast_state = use_context::<Signal<ToastState>>();

    rsx! {
        SectionTitle { title: "Toast" }
        div {
            class: "flex gap-3",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast_state.write().add("Event created", "Sunday, December 03, 2023 at 9:00 AM", ToastVariant::Default);
                },
                "Show Toast"
            }
            Button {
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    toast_state.write().add("Error", "Something went wrong!", ToastVariant::Destructive);
                },
                "Show Error Toast"
            }
        }
    }
}
