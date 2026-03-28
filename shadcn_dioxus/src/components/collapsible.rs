use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Collapsible(
    #[props(default)] open: bool,
    #[props(default)] class: String,
    onchange: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(CollapsibleContext {
        open,
        onchange: onchange.clone(),
    }));

    let classes = cn(&["", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[derive(Clone)]
pub struct CollapsibleContext {
    pub open: bool,
    pub onchange: Option<EventHandler<bool>>,
}

#[component]
pub fn CollapsibleTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<CollapsibleContext>>();
    let is_open = ctx.read().open;
    let onchange = ctx.read().onchange.clone();

    rsx! {
        div {
            class: "{class}",
            onclick: move |_| {
                if let Some(handler) = &onchange {
                    handler.call(!is_open);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn CollapsibleContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<CollapsibleContext>>();
    let is_open = ctx.read().open;

    if !is_open {
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
