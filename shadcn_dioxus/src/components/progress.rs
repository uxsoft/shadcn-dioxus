use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Progress(
    #[props(default = 0.0)] value: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default)] class: String,
) -> Element {
    let percentage = if max > 0.0 { (value / max) * 100.0 } else { 0.0 };
    let transform = format!("transform: translateX(-{}%);", 100.0 - percentage);
    let classes = cn(&["relative h-2 w-full overflow-hidden rounded-full bg-primary/20", &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "progressbar",
            "aria-valuemin": "0",
            "aria-valuemax": "{max}",
            "aria-valuenow": "{value}",
            div {
                class: "h-full w-full flex-1 bg-primary transition-all",
                style: "{transform}",
            }
        }
    }
}
