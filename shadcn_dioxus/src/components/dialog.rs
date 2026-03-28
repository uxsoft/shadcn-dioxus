use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Dialog(
    #[props(default)] open: bool,
    #[props(default)] class: String,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(DialogContext {
        open,
        onclose: onclose.clone(),
    }));

    if !open {
        return rsx! {};
    }

    rsx! {
        {children}
    }
}

#[derive(Clone)]
pub struct DialogContext {
    pub open: bool,
    pub onclose: Option<EventHandler<()>>,
}

#[component]
pub fn DialogTrigger(
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
pub fn DialogOverlay(
    #[props(default)] class: String,
) -> Element {
    let ctx = use_context::<Signal<DialogContext>>();
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

#[component]
pub fn DialogContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<DialogContext>>();
    let onclose = ctx.read().onclose.clone();

    let classes = cn(&[
        "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg sm:max-w-lg",
        "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
        &class,
    ]);
    rsx! {
        DialogOverlay {}
        div {
            class: "{classes}",
            "data-state": "open",
            role: "dialog",
            onclick: move |evt| evt.stop_propagation(),
            {children}
            // Close button
            button {
                r#type: "button",
                class: "absolute top-4 right-4 rounded-xs opacity-70 transition-opacity cursor-pointer hover:opacity-100 focus:ring-[3px] focus:ring-ring/50 focus:outline-none disabled:pointer-events-none",
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
pub fn DialogHeader(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex flex-col gap-2 text-center sm:text-left", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn DialogFooter(
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
pub fn DialogTitle(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["text-lg font-semibold leading-none", &class]);
    rsx! {
        h2 {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn DialogDescription(
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
