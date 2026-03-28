use dioxus::prelude::*;
use super::utils::{cn, Side};

#[derive(Clone)]
pub struct DrawerContext {
    pub open: bool,
    pub side: Side,
    pub onclose: Option<EventHandler<()>>,
}

#[component]
pub fn Drawer(
    #[props(default)] open: bool,
    #[props(default = Side::Bottom)] side: Side,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(DrawerContext {
        open,
        side,
        onclose: onclose.clone(),
    }));

    if !open {
        return rsx! {};
    }

    rsx! {
        {children}
    }
}

#[component]
pub fn DrawerOverlay(
    #[props(default)] class: String,
) -> Element {
    let ctx = use_context::<Signal<DrawerContext>>();
    let onclose = ctx.read().onclose.clone();
    let classes = cn(&[
        "fixed inset-0 z-50 bg-black/50",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            onclick: move |_| {
                if let Some(handler) = &onclose {
                    handler.call(());
                }
            },
        }
    }
}

fn drawer_side_classes(side: Side) -> &'static str {
    match side {
        Side::Top => "inset-x-0 top-0 mb-24 rounded-b-lg border-b",
        Side::Bottom => "inset-x-0 bottom-0 mt-24 rounded-t-lg border-t",
        Side::Left => "inset-y-0 left-0 w-3/4 sm:max-w-sm rounded-r-lg border-r",
        Side::Right => "inset-y-0 right-0 w-3/4 sm:max-w-sm rounded-l-lg border-l",
    }
}

#[component]
pub fn DrawerContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<DrawerContext>>();
    let side = ctx.read().side;
    let classes = cn(&[
        "fixed z-50 flex flex-col bg-background",
        drawer_side_classes(side),
        &class,
    ]);

    rsx! {
        DrawerOverlay {}
        div {
            class: "{classes}",
            onclick: move |evt| evt.stop_propagation(),
            if matches!(side, Side::Bottom) {
                div {
                    class: "mx-auto mt-4 h-2 w-[100px] rounded-full bg-muted",
                }
            }
            {children}
        }
    }
}

#[component]
pub fn DrawerHeader(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["grid gap-1.5 p-4 text-center sm:text-left", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn DrawerFooter(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["mt-auto flex flex-col gap-2 p-4", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn DrawerTitle(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["text-lg font-semibold leading-none tracking-tight", &class]);
    rsx! {
        h2 {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn DrawerDescription(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["text-sm text-muted-foreground", &class]);
    rsx! {
        p {
            class: "{classes}",
            {children}
        }
    }
}
