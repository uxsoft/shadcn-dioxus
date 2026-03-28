use dioxus::prelude::*;
use crate::components::*;
use crate::components::utils::Side;
use super::section_title::SectionTitle;

#[component]
pub fn SheetShowcase() -> Element {
    let mut sheet_open = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Sheet" }
        Button {
            variant: ButtonVariant::Outline,
            onclick: move |_| sheet_open.set(true),
            "Open Sheet"
        }
        Sheet {
            open: sheet_open(),
            side: Side::Right,
            onclose: move |_| sheet_open.set(false),
            SheetContent {
                SheetHeader {
                    SheetTitle { "Edit profile" }
                    SheetDescription { "Make changes to your profile here." }
                }
                div {
                    class: "grid gap-4 py-4",
                    Input { placeholder: "Name" }
                    Input { placeholder: "Username" }
                }
                SheetFooter {
                    Button {
                        onclick: move |_| sheet_open.set(false),
                        "Save changes"
                    }
                }
            }
        }
    }
}
