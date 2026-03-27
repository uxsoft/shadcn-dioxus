use dioxus::prelude::*;
use super::utils::{cn, Side};

#[derive(Clone)]
pub struct SidebarContext {
    pub open: bool,
    pub side: Side,
    pub onchange: Option<EventHandler<bool>>,
}

#[component]
pub fn SidebarProvider(
    #[props(default = true)] open: bool,
    #[props(default = Side::Left)] side: Side,
    #[props(default)] class: String,
    onchange: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(SidebarContext {
        open,
        side,
        onchange: onchange.clone(),
    }));

    let classes = cn(&[
        "flex min-h-screen w-full",
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
pub fn Sidebar(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<SidebarContext>>();
    let is_open = ctx.read().open;
    let side = ctx.read().side;

    let side_class = match side {
        Side::Left => "border-r",
        Side::Right => "border-l order-last",
        _ => "border-r",
    };

    let width_class = if is_open {
        "w-64"
    } else {
        "w-0 overflow-hidden"
    };

    let classes = cn(&[
        "relative flex flex-col bg-sidebar text-sidebar-foreground transition-all duration-300",
        side_class,
        width_class,
        &class,
    ]);

    rsx! {
        aside {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SidebarHeader(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex flex-col gap-2 p-4", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SidebarContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex min-h-0 flex-1 flex-col gap-2 overflow-auto p-4", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SidebarFooter(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex flex-col gap-2 p-4", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SidebarGroup(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["relative flex w-full min-w-0 flex-col gap-1", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SidebarGroupLabel(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium text-sidebar-foreground/70 outline-none transition-[margin,opa] duration-200",
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
pub fn SidebarGroupContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["w-full text-sm", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SidebarMenu(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex w-full min-w-0 flex-col gap-1", &class]);
    rsx! {
        ul {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SidebarMenuItem(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["group/menu-item relative", &class]);
    rsx! {
        li {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SidebarMenuButton(
    #[props(default)] active: bool,
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let active_class = if active {
        "bg-sidebar-accent text-sidebar-accent-foreground font-medium"
    } else {
        ""
    };
    let classes = cn(&[
        "flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-none transition-[width,height,padding] cursor-pointer",
        "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
        "focus-visible:ring-2 focus-visible:ring-sidebar-ring",
        "[&>svg]:size-4 [&>svg]:shrink-0",
        active_class,
        &class,
    ]);
    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            "data-active": if active { "true" } else { "" },
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
pub fn SidebarTrigger(
    #[props(default)] class: String,
) -> Element {
    let ctx = use_context::<Signal<SidebarContext>>();
    let is_open = ctx.read().open;
    let onchange = ctx.read().onchange.clone();
    let classes = cn(&[
        "inline-flex items-center justify-center size-8 rounded-md cursor-pointer hover:bg-accent hover:text-accent-foreground",
        &class,
    ]);
    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            onclick: move |_| {
                if let Some(handler) = &onchange {
                    handler.call(!is_open);
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
                rect { x: "3", y: "3", width: "18", height: "18", rx: "2" }
                path { d: "M9 3v18" }
            }
        }
    }
}

#[component]
pub fn SidebarInset(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "relative flex min-h-svh flex-1 flex-col bg-background",
        &class,
    ]);
    rsx! {
        main {
            class: "{classes}",
            {children}
        }
    }
}
