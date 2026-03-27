use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct DropdownMenuContext {
    pub open: bool,
    pub onclose: Option<EventHandler<()>>,
}

#[component]
pub fn DropdownMenu(
    #[props(default)] open: bool,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(DropdownMenuContext {
        open,
        onclose: onclose.clone(),
    }));

    rsx! {
        div {
            class: "relative inline-block",
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuTrigger(
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
pub fn DropdownMenuContent(
    #[props(default)] class: String,
    #[props(default = "bottom".to_string())] align: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<DropdownMenuContext>>();
    if !ctx.read().open {
        return rsx! {};
    }
    let onclose = ctx.read().onclose.clone();

    let align_class = match align.as_str() {
        "start" | "left" => "left-0",
        "end" | "right" => "right-0",
        _ => "left-0",
    };

    let classes = cn(&[
        "absolute z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md",
        "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
        align_class,
        "mt-1",
        &class,
    ]);

    rsx! {
        // Invisible overlay to catch outside clicks
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| {
                if let Some(handler) = &onclose {
                    handler.call(());
                }
            },
        }
        div {
            class: "{classes}",
            "data-state": "open",
            onclick: move |evt| evt.stop_propagation(),
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuItem(
    #[props(default)] class: String,
    #[props(default)] disabled: bool,
    #[props(default)] destructive: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let color = if destructive {
        "text-destructive focus:text-destructive"
    } else {
        ""
    };
    let classes = cn(&[
        "relative flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm cursor-pointer outline-none transition-colors select-none",
        "hover:bg-accent hover:text-accent-foreground",
        "focus:bg-accent focus:text-accent-foreground",
        "data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        "[&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        color,
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            role: "menuitem",
            "data-disabled": if disabled { "true" } else { "" },
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
pub fn DropdownMenuSeparator(
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
pub fn DropdownMenuLabel(
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
pub fn DropdownMenuShortcut(
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

#[component]
pub fn DropdownMenuGroup(
    children: Element,
) -> Element {
    rsx! {
        div {
            role: "group",
            {children}
        }
    }
}
