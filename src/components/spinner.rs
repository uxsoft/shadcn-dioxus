use dioxus::prelude::*;
use super::utils::cn;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Default,
    Lg,
}

fn size_class(size: SpinnerSize) -> &'static str {
    match size {
        SpinnerSize::Sm => "size-4",
        SpinnerSize::Default => "size-6",
        SpinnerSize::Lg => "size-8",
    }
}

#[component]
pub fn Spinner(
    #[props(default)] size: SpinnerSize,
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&["animate-spin text-muted-foreground", size_class(size), &class]);
    rsx! {
        svg {
            class: "{classes}",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            circle {
                class: "opacity-25",
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "4",
            }
            path {
                class: "opacity-75",
                fill: "currentColor",
                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z",
            }
        }
    }
}
