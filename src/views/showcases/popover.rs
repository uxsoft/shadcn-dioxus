use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn PopoverShowcase() -> Element {
    let mut popover_open = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Popover" }
        Popover {
            open: popover_open(),
            onclose: move |_| popover_open.set(false),
            PopoverTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| popover_open.set(!popover_open()),
                    "Open popover"
                }
            }
            PopoverContent {
                div {
                    class: "grid gap-4",
                    div {
                        class: "space-y-2",
                        h4 { class: "font-medium leading-none", "Dimensions" }
                        p { class: "text-sm text-muted-foreground", "Set the dimensions for the layer." }
                    }
                }
            }
        }
    }
}
