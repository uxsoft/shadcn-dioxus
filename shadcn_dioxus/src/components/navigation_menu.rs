use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct NavigationMenuContext {
    pub active_item: String,
}

#[component]
pub fn NavigationMenu(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_signal(|| NavigationMenuContext {
        active_item: String::new(),
    });
    use_context_provider(|| ctx);

    let classes = cn(&[
        "group relative flex max-w-max flex-1 items-center justify-center",
        &class,
    ]);

    rsx! {
        nav {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn NavigationMenuList(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "group flex flex-1 list-none items-center justify-center gap-1",
        &class,
    ]);
    rsx! {
        ul {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn NavigationMenuItem(
    #[props(default)] value: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(NavigationMenuItemContext { value: value.clone() }));
    rsx! {
        li {
            class: "relative {class}",
            {children}
        }
    }
}

#[derive(Clone)]
pub struct NavigationMenuItemContext {
    pub value: String,
}

#[component]
pub fn NavigationMenuTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<NavigationMenuContext>>();
    let item_ctx = use_context::<Signal<NavigationMenuItemContext>>();
    let item_value = item_ctx.read().value.clone();
    let is_open = ctx.read().active_item == item_value;

    let active_class = if is_open { "bg-accent/50 text-accent-foreground" } else { "" };
    let classes = cn(&[
        "group inline-flex h-9 w-max items-center justify-center gap-1 rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors cursor-pointer",
        "hover:bg-accent hover:text-accent-foreground",
        "focus:bg-accent focus:text-accent-foreground focus:outline-none",
        "disabled:pointer-events-none disabled:opacity-50",
        active_class,
        &class,
    ]);

    let chevron_rotation = if is_open { "rotate-180" } else { "" };

    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            onclick: move |_| {
                if is_open {
                    ctx.write().active_item = String::new();
                } else {
                    ctx.write().active_item = item_value.clone();
                }
            },
            {children}
            svg {
                class: "ml-1 size-3 transition-transform duration-200 {chevron_rotation}",
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
pub fn NavigationMenuContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<NavigationMenuContext>>();
    let item_ctx = use_context::<Signal<NavigationMenuItemContext>>();
    let item_value = item_ctx.read().value.clone();
    let is_open = ctx.read().active_item == item_value;

    if !is_open {
        return rsx! {};
    }

    let classes = cn(&[
        "absolute left-0 top-full z-50 mt-1.5 w-auto min-w-[12rem] rounded-md border bg-popover p-4 text-popover-foreground shadow-lg",
        "animate-in fade-in-0 zoom-in-95",
        &class,
    ]);

    rsx! {
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| {
                ctx.write().active_item = String::new();
            },
        }
        div {
            class: "{classes}",
            onclick: move |evt| evt.stop_propagation(),
            {children}
        }
    }
}

#[component]
pub fn NavigationMenuLink(
    #[props(default)] href: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors",
        "hover:bg-accent hover:text-accent-foreground",
        "focus:bg-accent focus:text-accent-foreground",
        &class,
    ]);
    rsx! {
        a {
            class: "{classes}",
            href,
            {children}
        }
    }
}
