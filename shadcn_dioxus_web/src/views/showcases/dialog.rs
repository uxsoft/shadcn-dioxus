use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn DialogShowcase() -> Element {
    let mut dialog_open = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Dialog" }
        Button {
            variant: ButtonVariant::Outline,
            onclick: move |_| dialog_open.set(true),
            "Open Dialog"
        }
        Dialog {
            open: dialog_open(),
            onclose: move |_| dialog_open.set(false),
            DialogContent {
                DialogHeader {
                    DialogTitle { "Edit profile" }
                    DialogDescription { "Make changes to your profile here. Click save when you're done." }
                }
                div {
                    class: "grid gap-4 py-4",
                    Input { placeholder: "Name" }
                    Input { placeholder: "Username" }
                }
                DialogFooter {
                    Button {
                        onclick: move |_| dialog_open.set(false),
                        "Save changes"
                    }
                }
            }
        }
    }
}
