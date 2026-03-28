use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn ScrollArea(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["relative overflow-auto", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}
