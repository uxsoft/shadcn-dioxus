use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn HoverCard(
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
pub fn HoverCardTrigger(
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
pub fn HoverCardContent(
    #[props(default)] class: String,
    #[props(default = "center".to_string())] align: String,
    children: Element,
) -> Element {
    let align_class = match align.as_str() {
        "start" | "left" => "left-0",
        "end" | "right" => "right-0",
        _ => "left-1/2 -translate-x-1/2",
    };

    let classes = cn(&[
        "absolute z-50 hidden group-hover:block w-64 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none",
        "animate-in fade-in-0 zoom-in-95",
        "mt-2 top-full",
        align_class,
        &class,
    ]);

    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}
