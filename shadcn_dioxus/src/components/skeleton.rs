use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Skeleton(
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&["animate-pulse rounded-md bg-accent", &class]);
    rsx! {
        div {
            class: "{classes}",
        }
    }
}
