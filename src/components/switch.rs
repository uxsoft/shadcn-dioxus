use dioxus::prelude::*;
use super::utils::cn;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SwitchSize {
    Sm,
    #[default]
    Default,
}

#[component]
pub fn Switch(
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    #[props(default)] size: SwitchSize,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
    onchange: Option<EventHandler<bool>>,
) -> Element {
    let (track_size, thumb_size, translate) = match size {
        SwitchSize::Sm => ("h-3.5 w-6", "size-3", if checked { "translate-x-[calc(100%-2px)]" } else { "translate-x-0" }),
        SwitchSize::Default => ("h-[1.15rem] w-8", "size-4", if checked { "translate-x-[calc(100%-2px)]" } else { "translate-x-0" }),
    };
    let bg = if checked {
        "bg-primary"
    } else {
        "bg-input dark:bg-input/80"
    };
    let track_classes = cn(&[
        "peer inline-flex shrink-0 items-center rounded-full border border-transparent shadow-xs transition-all outline-none cursor-pointer",
        "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:cursor-not-allowed disabled:opacity-50",
        track_size,
        bg,
        &class,
    ]);
    let thumb_classes = cn(&[
        "pointer-events-none block rounded-full bg-background ring-0 transition-transform",
        thumb_size,
        translate,
        if checked { "dark:bg-primary-foreground" } else { "dark:bg-foreground" },
    ]);
    rsx! {
        button {
            r#type: "button",
            role: "switch",
            class: "{track_classes}",
            id,
            disabled,
            "aria-checked": if checked { "true" } else { "false" },
            onclick: move |_| {
                if let Some(handler) = &onchange {
                    handler.call(!checked);
                }
            },
            span {
                class: "{thumb_classes}",
            }
        }
    }
}
