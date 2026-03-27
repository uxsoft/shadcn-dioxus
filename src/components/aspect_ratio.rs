use dioxus::prelude::*;

#[component]
pub fn AspectRatio(
    #[props(default = 1.0)] ratio: f64,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let padding = format!("padding-bottom: {}%;", 100.0 / ratio);
    rsx! {
        div {
            class: "relative w-full {class}",
            style: "{padding}",
            div {
                class: "absolute inset-0",
                {children}
            }
        }
    }
}
