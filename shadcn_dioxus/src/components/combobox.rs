use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct ComboboxContext {
    pub value: String,
    pub search: String,
    pub open: bool,
    pub onchange: Option<EventHandler<String>>,
}

#[component]
pub fn Combobox(
    #[props(default)] value: String,
    #[props(default)] class: String,
    onchange: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let ctx = use_signal(|| ComboboxContext {
        value: value.clone(),
        search: String::new(),
        open: false,
        onchange: onchange.clone(),
    });
    use_context_provider(|| ctx);

    rsx! {
        div {
            class: "relative {class}",
            {children}
        }
    }
}

#[component]
pub fn ComboboxTrigger(
    #[props(default)] placeholder: String,
    #[props(default)] class: String,
) -> Element {
    let mut ctx = use_context::<Signal<ComboboxContext>>();
    let is_open = ctx.read().open;
    let current_value = ctx.read().value.clone();

    let classes = cn(&[
        "flex h-9 w-full items-center justify-between gap-2 rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-xs cursor-pointer",
        "hover:bg-accent/50 focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "dark:bg-input/30",
        &class,
    ]);

    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            onclick: move |_| {
                ctx.write().open = !is_open;
            },
            span {
                class: if current_value.is_empty() { "text-muted-foreground" } else { "" },
                if current_value.is_empty() {
                    "{placeholder}"
                } else {
                    "{current_value}"
                }
            }
            svg {
                class: "size-4 opacity-50 shrink-0",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }
}

#[component]
pub fn ComboboxContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<ComboboxContext>>();
    if !ctx.read().open {
        return rsx! {};
    }

    let search = ctx.read().search.clone();
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
                ctx.write().search = String::new();
            },
        }
        div {
            class: "{classes}",
            onclick: move |evt| evt.stop_propagation(),
            // Search input
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
                    class: "flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground",
                    placeholder: "Search...",
                    value: "{search}",
                    oninput: move |e| {
                        ctx.write().search = e.value();
                    },
                }
            }
            div {
                class: "max-h-[300px] overflow-y-auto p-1",
                {children}
            }
        }
    }
}

#[component]
pub fn ComboboxItem(
    value: String,
    #[props(default)] label: String,
    #[props(default)] class: String,
) -> Element {
    let mut ctx = use_context::<Signal<ComboboxContext>>();
    let is_selected = ctx.read().value == value;
    let search = ctx.read().search.clone();
    let onchange = ctx.read().onchange.clone();
    let display = if label.is_empty() { &value } else { &label };
    let value_clone = value.clone();

    // Filter by search
    if !search.is_empty() && !display.to_lowercase().contains(&search.to_lowercase()) {
        return rsx! {};
    }

    let classes = cn(&[
        "relative flex items-center rounded-sm py-1.5 pl-2 pr-8 text-sm cursor-pointer outline-none select-none",
        "hover:bg-accent hover:text-accent-foreground",
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
                ctx.write().search = String::new();
            },
            "{display}"
            if is_selected {
                span {
                    class: "absolute right-2 flex items-center",
                    svg {
                        class: "size-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M20 6 9 17l-5-5" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ComboboxEmpty(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["py-6 text-center text-sm text-muted-foreground", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}
