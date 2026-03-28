use dioxus::prelude::*;
use super::utils::{cn, Side};

#[derive(Clone)]
pub struct SheetContext {
    pub open: bool,
    pub side: Side,
    pub onclose: Option<EventHandler<()>>,
}

#[component]
pub fn Sheet(
    #[props(default)] open: bool,
    #[props(default)] side: Side,
    #[props(default)] class: String,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(SheetContext {
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
pub fn SheetOverlay(
    #[props(default)] class: String,
) -> Element {
    let ctx = use_context::<Signal<SheetContext>>();
    let onclose = ctx.read().onclose.clone();
    let classes = cn(&[
        "fixed inset-0 z-50 bg-black/50 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            "data-state": "open",
            onclick: move |_| {
                if let Some(handler) = &onclose {
                    handler.call(());
                }
            },
        }
    }
}

fn side_classes(side: Side) -> &'static str {
    match side {
        Side::Top => "inset-x-0 top-0 border-b data-[state=open]:slide-in-from-top data-[state=closed]:slide-out-to-top",
        Side::Bottom => "inset-x-0 bottom-0 border-t data-[state=open]:slide-in-from-bottom data-[state=closed]:slide-out-to-bottom",
        Side::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm data-[state=open]:slide-in-from-left data-[state=closed]:slide-out-to-left",
        Side::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm data-[state=open]:slide-in-from-right data-[state=closed]:slide-out-to-right",
    }
}

#[component]
pub fn SheetContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<SheetContext>>();
    let side = ctx.read().side;
    let onclose = ctx.read().onclose.clone();

    let classes = cn(&[
        "fixed z-50 flex flex-col gap-4 bg-background p-6 shadow-lg transition ease-in-out data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:duration-300 data-[state=open]:duration-500",
        side_classes(side),
        &class,
    ]);

    rsx! {
        SheetOverlay {}
        div {
            class: "{classes}",
            "data-state": "open",
            role: "dialog",
            onclick: move |evt| evt.stop_propagation(),
            {children}
            // Close button
            button {
                r#type: "button",
                class: "absolute top-4 right-4 rounded-xs opacity-70 transition-opacity cursor-pointer hover:opacity-100 focus:ring-[3px] focus:ring-ring/50 focus:outline-none",
                onclick: move |_| {
                    if let Some(handler) = &onclose {
                        handler.call(());
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
                    path { d: "M18 6 6 18" }
                    path { d: "m6 6 12 12" }
                }
                span {
                    class: "sr-only",
                    "Close"
                }
            }
        }
    }
}

#[component]
pub fn SheetHeader(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex flex-col gap-2", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SheetFooter(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex flex-col-reverse gap-2 sm:flex-row sm:justify-end", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SheetTitle(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["text-lg font-semibold", &class]);
    rsx! {
        h2 {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn SheetDescription(
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
