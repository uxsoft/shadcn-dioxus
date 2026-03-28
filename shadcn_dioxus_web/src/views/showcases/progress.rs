use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn ProgressShowcase() -> Element {
    let mut progress_val = use_signal(|| 60.0_f64);

    rsx! {
        SectionTitle { title: "Progress" }
        div {
            class: "max-w-sm space-y-2",
            Progress { value: progress_val() }
            div {
                class: "flex gap-2",
                Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        let v = (progress_val() - 10.0).max(0.0);
                        progress_val.set(v);
                    },
                    "-10"
                }
                Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        let v = (progress_val() + 10.0).min(100.0);
                        progress_val.set(v);
                    },
                    "+10"
                }
            }
        }
    }
}
