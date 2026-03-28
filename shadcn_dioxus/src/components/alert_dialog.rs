use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct AlertDialogContext {
    pub open: bool,
    pub onclose: Option<EventHandler<()>>,
}

#[component]
pub fn AlertDialog(
    #[props(default)] open: bool,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(AlertDialogContext {
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

#[component]
pub fn AlertDialogOverlay(
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&[
        "fixed inset-0 z-50 bg-black/50 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            "data-state": "open",
        }
    }
}

#[component]
pub fn AlertDialogContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg sm:max-w-lg",
        "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
        &class,
    ]);
    rsx! {
        AlertDialogOverlay {}
        div {
            class: "{classes}",
            "data-state": "open",
            role: "alertdialog",
            {children}
        }
    }
}

#[component]
pub fn AlertDialogHeader(
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
pub fn AlertDialogFooter(
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
pub fn AlertDialogTitle(
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
pub fn AlertDialogDescription(
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

#[component]
pub fn AlertDialogAction(
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let classes = cn(&[
        "inline-flex h-9 items-center justify-center gap-2 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground shadow-xs transition-colors cursor-pointer hover:bg-primary/90 focus-visible:ring-[3px] focus-visible:ring-ring/50 outline-none",
        &class,
    ]);
    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
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
pub fn AlertDialogCancel(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<AlertDialogContext>>();
    let onclose = ctx.read().onclose.clone();
    let classes = cn(&[
        "inline-flex h-9 items-center justify-center gap-2 rounded-md border bg-background px-4 text-sm font-medium shadow-xs transition-colors cursor-pointer hover:bg-accent hover:text-accent-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 outline-none",
        &class,
    ]);
    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            onclick: move |_| {
                if let Some(handler) = &onclose {
                    handler.call(());
                }
            },
            {children}
        }
    }
}
