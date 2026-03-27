use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct PopoverContext {
    pub open: bool,
    pub onclose: Option<EventHandler<()>>,
}

#[component]
pub fn Popover(
    #[props(default)] open: bool,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(PopoverContext {
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
pub fn PopoverTrigger(
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
pub fn PopoverContent(
    #[props(default)] class: String,
    #[props(default = "center".to_string())] align: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<PopoverContext>>();
    if !ctx.read().open {
        return rsx! {};
    }
    let onclose = ctx.read().onclose.clone();

    let align_class = match align.as_str() {
        "start" | "left" => "left-0",
        "end" | "right" => "right-0",
        _ => "left-1/2 -translate-x-1/2",
    };

    let classes = cn(&[
        "absolute z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none",
        "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
        "mt-2",
        align_class,
        &class,
    ]);

    rsx! {
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
