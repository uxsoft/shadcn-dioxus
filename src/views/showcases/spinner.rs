use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn SpinnerShowcase() -> Element {
    rsx! {
        SectionTitle { title: "Spinner" }
        div {
            class: "flex gap-4 items-center",
            Spinner { size: SpinnerSize::Sm }
            Spinner {}
            Spinner { size: SpinnerSize::Lg }
        }
    }
}
