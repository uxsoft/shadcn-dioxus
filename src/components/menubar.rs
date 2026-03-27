use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct MenubarContext {
    pub active_menu: String,
}

#[component]
pub fn Menubar(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_signal(|| MenubarContext {
        active_menu: String::new(),
    });
    use_context_provider(|| ctx);

    let classes = cn(&[
        "flex h-9 items-center gap-1 rounded-md border bg-background p-1 shadow-xs",
        &class,
    ]);

    rsx! {
        div {
            class: "{classes}",
            role: "menubar",
            {children}
        }
    }
}

#[component]
pub fn MenubarMenu(
    value: String,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(MenubarMenuContext { value: value.clone() }));
    rsx! {
        div {
            class: "relative",
            {children}
        }
    }
}

#[derive(Clone)]
pub struct MenubarMenuContext {
    pub value: String,
}

#[component]
pub fn MenubarTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<MenubarContext>>();
    let menu_ctx = use_context::<Signal<MenubarMenuContext>>();
    let menu_value = menu_ctx.read().value.clone();
    let is_open = ctx.read().active_menu == menu_value;

    let active_class = if is_open { "bg-accent text-accent-foreground" } else { "" };
    let classes = cn(&[
        "flex items-center rounded-sm px-2 py-1 text-sm font-medium cursor-pointer outline-none select-none",
        "hover:bg-accent hover:text-accent-foreground",
        "focus:bg-accent focus:text-accent-foreground",
        active_class,
        &class,
    ]);

    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            onclick: move |_| {
                if is_open {
                    ctx.write().active_menu = String::new();
                } else {
                    ctx.write().active_menu = menu_value.clone();
                }
            },
            {children}
        }
    }
}

#[component]
pub fn MenubarContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<MenubarContext>>();
    let menu_ctx = use_context::<Signal<MenubarMenuContext>>();
    let menu_value = menu_ctx.read().value.clone();
    let is_open = ctx.read().active_menu == menu_value;

    if !is_open {
        return rsx! {};
    }

    let classes = cn(&[
        "absolute left-0 z-50 mt-1 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md",
        "animate-in fade-in-0 zoom-in-95",
        &class,
    ]);

    rsx! {
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| {
                ctx.write().active_menu = String::new();
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
pub fn MenubarItem(
    #[props(default)] class: String,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<MenubarContext>>();
    let classes = cn(&[
        "relative flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm cursor-pointer outline-none select-none",
        "hover:bg-accent hover:text-accent-foreground",
        "[&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            role: "menuitem",
            onclick: move |evt| {
                if !disabled {
                    if let Some(handler) = &onclick {
                        handler.call(evt);
                    }
                    ctx.write().active_menu = String::new();
                }
            },
            {children}
        }
    }
}

#[component]
pub fn MenubarSeparator(
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&["-mx-1 my-1 h-px bg-border", &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "separator",
        }
    }
}

#[component]
pub fn MenubarShortcut(
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
