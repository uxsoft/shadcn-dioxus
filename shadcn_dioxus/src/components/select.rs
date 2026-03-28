use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct SelectContext {
    pub value: String,
    pub open: bool,
    pub onchange: Option<EventHandler<String>>,
}

#[component]
pub fn Select(
    #[props(default)] value: String,
    #[props(default)] class: String,
    onchange: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let mut ctx = use_signal(|| SelectContext {
        value: value.clone(),
        open: false,
        onchange: onchange.clone(),
    });

    use_context_provider(|| ctx);

    // Keep context in sync with props
    if ctx.read().value != value {
        ctx.write().value = value;
    }

    rsx! {
        div {
            class: "relative inline-block {class}",
            {children}
        }
    }
}

#[component]
pub fn SelectTrigger(
    #[props(default)] class: String,
    #[props(default)] placeholder: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<SelectContext>>();
    let is_open = ctx.read().open;
    let classes = cn(&[
        "flex h-9 w-full items-center justify-between gap-2 rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-xs transition-[color,box-shadow] outline-none cursor-pointer whitespace-nowrap",
        "hover:bg-accent/50",
        "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:cursor-not-allowed disabled:opacity-50",
        "[&>span]:line-clamp-1",
        "dark:bg-input/30",
        &class,
    ]);

    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            role: "combobox",
            "aria-expanded": if is_open { "true" } else { "false" },
            onclick: move |_| {
                ctx.write().open = !is_open;
            },
            {children}
            // Chevron
            svg {
                class: "size-4 opacity-50 shrink-0",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }
}

#[component]
pub fn SelectValue(
    #[props(default)] placeholder: String,
) -> Element {
    let ctx = use_context::<Signal<SelectContext>>();
    let value = ctx.read().value.clone();
    rsx! {
        span {
            if value.is_empty() {
                span {
                    class: "text-muted-foreground",
                    "{placeholder}"
                }
            } else {
                "{value}"
            }
        }
    }
}

#[component]
pub fn SelectContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<SelectContext>>();
    if !ctx.read().open {
        return rsx! {};
    }

    let classes = cn(&[
        "absolute z-50 mt-1 w-full min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md",
        "animate-in fade-in-0 zoom-in-95",
        &class,
    ]);

    rsx! {
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| {
                ctx.write().open = false;
            },
        }
        div {
            class: "{classes}",
            onclick: move |evt| evt.stop_propagation(),
            div {
                class: "p-1",
                {children}
            }
        }
    }
}

#[component]
pub fn SelectGroup(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "{class}",
            role: "group",
            {children}
        }
    }
}

#[component]
pub fn SelectLabel(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["px-2 py-1.5 text-sm font-semibold", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SelectItem(
    value: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<SelectContext>>();
    let is_selected = ctx.read().value == value;
    let onchange = ctx.read().onchange.clone();
    let value_clone = value.clone();

    let classes = cn(&[
        "relative flex w-full items-center rounded-sm py-1.5 pl-2 pr-8 text-sm cursor-pointer outline-none select-none",
        "hover:bg-accent hover:text-accent-foreground",
        "focus:bg-accent focus:text-accent-foreground",
        "data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        &class,
    ]);

    rsx! {
        div {
            class: "{classes}",
            role: "option",
            "aria-selected": if is_selected { "true" } else { "false" },
            onclick: move |_| {
                if let Some(handler) = &onchange {
                    handler.call(value_clone.clone());
                }
                ctx.write().open = false;
            },
            {children}
            if is_selected {
                span {
                    class: "absolute right-2 flex size-3.5 items-center justify-center",
                    svg {
                        class: "size-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M20 6 9 17l-5-5" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SelectSeparator(
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&["-mx-1 my-1 h-px bg-border", &class]);
    rsx! {
        div {
            class: "{classes}",
        }
    }
}
