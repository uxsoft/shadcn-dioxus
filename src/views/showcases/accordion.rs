use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn AccordionShowcase() -> Element {
    let mut accordion_val = use_signal(|| vec!["item-1".to_string()]);

    rsx! {
        SectionTitle { title: "Accordion" }
        div {
            class: "max-w-md",
            Accordion {
                value: accordion_val(),
                onchange: move |v| accordion_val.set(v),
                AccordionItem {
                    value: "item-1",
                    AccordionTrigger { "Is it accessible?" }
                    AccordionContent { "Yes. It adheres to the WAI-ARIA design pattern." }
                }
                AccordionItem {
                    value: "item-2",
                    AccordionTrigger { "Is it styled?" }
                    AccordionContent { "Yes. It comes with default styles that match the other components' aesthetic." }
                }
                AccordionItem {
                    value: "item-3",
                    AccordionTrigger { "Is it animated?" }
                    AccordionContent { "Yes. It's animated by default, but you can disable it if you prefer." }
                }
            }
        }
    }
}
