use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn SeparatorShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Separator" }
        div {
            class: "space-y-1",
            h4 { class: "text-sm font-medium leading-none", "shadcn/dioxus" }
            p { class: "text-sm text-muted-foreground", "A component library for Dioxus." }
            Separator {}
            p { class: "text-sm text-muted-foreground", "Built with Tailwind CSS." }
        }
    }
}
