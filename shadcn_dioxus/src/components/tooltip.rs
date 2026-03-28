use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Tooltip(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "relative inline-block group {class}",
            {children}
        }
    }
}

#[component]
pub fn TooltipTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "{class}",
            {children}
        }
    }
}

#[component]
pub fn TooltipContent(
    #[props(default)] class: String,
    #[props(default = "top".to_string())] side: String,
    children: Element,
) -> Element {
    let side_class = match side.as_str() {
        "bottom" => "top-full left-1/2 -translate-x-1/2 mt-2",
        "left" => "right-full top-1/2 -translate-y-1/2 mr-2",
        "right" => "left-full top-1/2 -translate-y-1/2 ml-2",
        _ => "bottom-full left-1/2 -translate-x-1/2 mb-2", // top default
    };

    let classes = cn(&[
        "absolute z-50 hidden group-hover:block w-fit overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground whitespace-nowrap",
        "animate-in fade-in-0 zoom-in-95",
        side_class,
        &class,
    ]);

    rsx! {
        div {
            class: "{classes}",
            role: "tooltip",
            {children}
        }
    }
}
