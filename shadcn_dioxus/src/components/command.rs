use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Command(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn CommandInput(
    #[props(default)] placeholder: String,
    #[props(default)] value: String,
    #[props(default)] class: String,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = cn(&[
        "flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
        &class,
    ]);
    rsx! {
        div {
            class: "flex items-center border-b px-3",
            svg {
                class: "mr-2 size-4 shrink-0 opacity-50",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                circle { cx: "11", cy: "11", r: "8" }
                path { d: "m21 21-4.3-4.3" }
            }
            input {
                class: "{classes}",
                placeholder,
                value,
                oninput: move |e| {
                    if let Some(handler) = &oninput {
                        handler.call(e);
                    }
                },
            }
        }
    }
}

#[component]
pub fn CommandList(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["max-h-[300px] overflow-y-auto overflow-x-hidden", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn CommandEmpty(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["py-6 text-center text-sm", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn CommandGroup(
    #[props(default)] heading: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "overflow-hidden p-1 text-foreground [&_[data-slot=command-item]]:py-1.5",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            if !heading.is_empty() {
                div {
                    class: "px-2 py-1.5 text-xs font-medium text-muted-foreground",
                    "{heading}"
                }
            }
            {children}
        }
    }
}

#[component]
pub fn CommandItem(
    #[props(default)] class: String,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let classes = cn(&[
        "relative flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm cursor-pointer outline-none select-none transition-colors",
        "hover:bg-accent hover:text-accent-foreground",
        "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        "[&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            "data-slot": "command-item",
            "data-disabled": if disabled { "true" } else { "false" },
            onclick: move |evt| {
                if !disabled {
                    if let Some(handler) = &onclick {
                        handler.call(evt);
                    }
                }
            },
            {children}
        }
    }
}

#[component]
pub fn CommandSeparator(
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&["-mx-1 h-px bg-border", &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "separator",
        }
    }
}

#[component]
pub fn CommandShortcut(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["ml-auto text-xs tracking-widest text-muted-foreground", &class]);
    rsx! {
        span {
            class: "{classes}",
            {children}
        }
    }
}
