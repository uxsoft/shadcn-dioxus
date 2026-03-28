use dioxus::prelude::*;
use super::utils::cn;

const BASE: &str = "peer size-4 shrink-0 rounded-[4px] border border-input shadow-xs transition-shadow outline-none cursor-pointer focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:bg-input/30 dark:aria-invalid:ring-destructive/40";

const CHECKED: &str = "border-primary bg-primary text-primary-foreground dark:bg-primary";
const UNCHECKED: &str = "";

#[component]
pub fn Checkbox(
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
    onchange: Option<EventHandler<bool>>,
) -> Element {
    let state_class = if checked { CHECKED } else { UNCHECKED };
    let classes = cn(&[BASE, state_class, &class]);

    rsx! {
        button {
            r#type: "button",
            role: "checkbox",
            class: "{classes}",
            id,
            disabled,
            "aria-checked": if checked { "true" } else { "false" },
            onclick: move |_| {
                if let Some(handler) = &onchange {
                    handler.call(!checked);
                }
            },
            if checked {
                svg {
                    class: "size-3.5 mx-auto",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path {
                        class: "animate-checkbox-check",
                        d: "M20 6 9 17l-5-5",
                    }
                }
            }
        }
    }
}
