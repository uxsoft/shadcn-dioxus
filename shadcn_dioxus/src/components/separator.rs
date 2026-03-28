use dioxus::prelude::*;
use super::utils::{cn, Orientation};

#[component]
pub fn Separator(
    #[props(default)] orientation: Orientation,
    #[props(default)] class: String,
) -> Element {
    let orientation_class = match orientation {
        Orientation::Horizontal => "h-px w-full",
        Orientation::Vertical => "h-full w-px",
    };
    let classes = cn(&["shrink-0 bg-border", orientation_class, &class]);
    rsx! {
        div {
            role: "separator",
            class: "{classes}",
        }
    }
}
