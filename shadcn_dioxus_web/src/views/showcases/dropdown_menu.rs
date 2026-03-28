use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn DropdownMenuShowcase() -> Element {
    let mut dropdown_open = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Dropdown Menu" }
        DropdownMenu {
            open: dropdown_open(),
            onclose: move |_| dropdown_open.set(false),
            DropdownMenuTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| dropdown_open.set(!dropdown_open()),
                    "Open Menu"
                }
            }
            DropdownMenuContent {
                DropdownMenuLabel { "My Account" }
                DropdownMenuSeparator {}
                DropdownMenuItem {
                    onclick: move |_| dropdown_open.set(false),
                    "Profile"
                }
                DropdownMenuItem {
                    onclick: move |_| dropdown_open.set(false),
                    "Billing"
                }
                DropdownMenuItem {
                    onclick: move |_| dropdown_open.set(false),
                    "Settings"
                }
                DropdownMenuSeparator {}
                DropdownMenuItem {
                    destructive: true,
                    onclick: move |_| dropdown_open.set(false),
                    "Log out"
                }
            }
        }
    }
}
