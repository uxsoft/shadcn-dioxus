use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn SliderShowcase() -> Element {
    let mut slider_val = use_signal(|| 50.0_f64);

    rsx! {
        SectionTitle { title: "Slider" }
        div {
            class: "max-w-sm space-y-2",
            Slider {
                value: slider_val(),
                min: 0.0,
                max: 100.0,
                step: 1.0,
                onchange: move |v| slider_val.set(v),
            }
            span { class: "text-sm text-muted-foreground", "Value: {slider_val}" }
        }
    }
}
