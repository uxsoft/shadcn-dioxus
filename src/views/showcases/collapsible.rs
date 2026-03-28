use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn CollapsibleShowcase() -> Element {
    let mut collapsible_open = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Collapsible" }
        div {
            class: "max-w-sm space-y-2",
            Collapsible {
                open: collapsible_open(),
                onchange: move |v| collapsible_open.set(v),
                div {
                    class: "flex items-center justify-between",
                    h4 { class: "text-sm font-semibold", "3 items starred" }
                    CollapsibleTrigger {
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            if collapsible_open() { "Hide" } else { "Show" }
                        }
                    }
                }
                div {
                    class: "rounded-md border px-4 py-2 text-sm",
                    "Item 1"
                }
                CollapsibleContent {
                    class: "space-y-2 mt-2",
                    div {
                        class: "rounded-md border px-4 py-2 text-sm",
                        "Item 2"
                    }
                    div {
                        class: "rounded-md border px-4 py-2 text-sm",
                        "Item 3"
                    }
                }
            }
        }
    }
}
