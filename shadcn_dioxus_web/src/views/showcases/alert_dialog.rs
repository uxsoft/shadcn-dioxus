use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn AlertDialogShowcase() -> Element {
    let mut alert_dialog_open = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Alert Dialog" }
        Button {
            variant: ButtonVariant::Outline,
            onclick: move |_| alert_dialog_open.set(true),
            "Open Alert Dialog"
        }
        AlertDialog {
            open: alert_dialog_open(),
            onclose: move |_| alert_dialog_open.set(false),
            AlertDialogContent {
                AlertDialogHeader {
                    AlertDialogTitle { "Are you absolutely sure?" }
                    AlertDialogDescription {
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    }
                }
                AlertDialogFooter {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction {
                        onclick: move |_| alert_dialog_open.set(false),
                        "Continue"
                    }
                }
            }
        }
    }
}
