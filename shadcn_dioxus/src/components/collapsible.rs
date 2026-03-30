use std::ops::Not;

use super::utils::cn;
use dioxus::prelude::*;

#[derive(Clone)]
pub struct CollapsibleContext {
    pub open: ReadSignal<bool>,
    pub onchange: Option<EventHandler<bool>>,
}

#[component]
pub fn Collapsible(
    #[props(default)] open: ReadSignal<bool>,
    #[props(default)] class: String,
    onchange: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(CollapsibleContext { open, onchange }));

    let classes = cn(&["", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn CollapsibleTrigger(#[props(default)] class: String, children: Element) -> Element {
    let ctx = use_context::<Signal<CollapsibleContext>>();

    rsx! {
        div {
            class: "{class}",
            onclick: move |_| {
                if let Some(handler) = ctx().onchange {
                    handler.call(ctx().open.read().not());
                }
            },
            {children}
        }
    }
}

#[component]
pub fn CollapsibleContent(#[props(default)] class: String, children: Element) -> Element {
    let ctx = use_context::<Signal<CollapsibleContext>>();

    if ctx().open.read().not() {
        return rsx! {};
    }

    let classes = cn(&["overflow-hidden text-sm", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}
