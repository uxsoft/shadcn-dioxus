use dioxus::prelude::*;
use super::utils::cn;
use super::button::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn Pagination(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["mx-auto flex w-full justify-center", &class]);
    rsx! {
        nav {
            class: "{classes}",
            role: "navigation",
            "aria-label": "pagination",
            {children}
        }
    }
}

#[component]
pub fn PaginationContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex flex-row items-center gap-1", &class]);
    rsx! {
        ul {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn PaginationItem(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        li {
            class: "{class}",
            {children}
        }
    }
}

#[component]
pub fn PaginationLink(
    #[props(default)] active: bool,
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant = if active { ButtonVariant::Outline } else { ButtonVariant::Ghost };
    rsx! {
        Button {
            variant,
            size: ButtonSize::Icon,
            class: "{class}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn PaginationPrevious(
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Default,
            class: "gap-1 pl-2.5 {class}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            svg {
                class: "size-4",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m15 18-6-6 6-6" }
            }
            span { "Previous" }
        }
    }
}

#[component]
pub fn PaginationNext(
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Default,
            class: "gap-1 pr-2.5 {class}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            span { "Next" }
            svg {
                class: "size-4",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}

#[component]
pub fn PaginationEllipsis(
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&["flex h-9 w-9 items-center justify-center", &class]);
    rsx! {
        span {
            class: "{classes}",
            "aria-hidden": "true",
            "..."
        }
    }
}
