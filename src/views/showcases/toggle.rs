use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn ToggleShowcase() -> Element {
    let mut toggle_pressed = use_signal(|| false);

    rsx! {
        SectionTitle { title: "Toggle" }
        div {
            class: "flex gap-3 items-center",
            Toggle {
                pressed: toggle_pressed(),
                onchange: move |v| toggle_pressed.set(v),
                svg {
                    class: "size-4",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                    path { d: "M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                }
            }
            Toggle {
                pressed: toggle_pressed(),
                onchange: move |v| toggle_pressed.set(v),
                variant: ToggleVariant::Outline,
                svg {
                    class: "size-4",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M4 7V4h16v3" }
                    path { d: "M9 20h6" }
                    path { d: "M12 4v16" }
                }
            }
        }
    }
}
