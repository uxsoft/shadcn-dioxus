use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct ContextMenuState {
    pub open: bool,
    pub x: f64,
    pub y: f64,
    pub onclose: Option<EventHandler<()>>,
}

#[component]
pub fn ContextMenu(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut state = use_signal(|| ContextMenuState {
        open: false,
        x: 0.0,
        y: 0.0,
        onclose: None,
    });

    use_context_provider(|| state);

    rsx! {
        div {
            class: "relative {class}",
            oncontextmenu: move |evt| {
                evt.prevent_default();
                let coords = evt.page_coordinates();
                state.write().open = true;
                state.write().x = coords.x;
                state.write().y = coords.y;
            },
            {children}
        }
    }
}

#[component]
pub fn ContextMenuTrigger(
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
pub fn ContextMenuContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut state = use_context::<Signal<ContextMenuState>>();
    if !state.read().open {
        return rsx! {};
    }

    let x = state.read().x;
    let y = state.read().y;

    let classes = cn(&[
        "fixed z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md",
        "animate-in fade-in-0 zoom-in-95",
        &class,
    ]);

    rsx! {
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| {
                state.write().open = false;
            },
        }
        div {
            class: "{classes}",
            style: "left: {x}px; top: {y}px;",
            onclick: move |evt| evt.stop_propagation(),
            {children}
        }
    }
}

#[component]
pub fn ContextMenuItem(
    #[props(default)] class: String,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut state = use_context::<Signal<ContextMenuState>>();
    let classes = cn(&[
        "relative flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm cursor-pointer outline-none transition-colors select-none",
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
                    state.write().open = false;
                }
            },
            {children}
        }
    }
}

#[component]
pub fn ContextMenuSeparator(
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
