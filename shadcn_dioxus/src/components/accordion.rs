use dioxus::prelude::*;
use super::utils::cn;

#[derive(Clone)]
pub struct AccordionContext {
    pub open_items: Vec<String>,
    pub multiple: bool,
    pub onchange: Option<EventHandler<Vec<String>>>,
}

#[component]
pub fn Accordion(
    #[props(default)] value: Vec<String>,
    #[props(default)] multiple: bool,
    #[props(default)] class: String,
    onchange: Option<EventHandler<Vec<String>>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(AccordionContext {
        open_items: value.clone(),
        multiple,
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

#[component]
pub fn AccordionItem(
    value: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(AccordionItemContext { value: value.clone() }));
    let classes = cn(&["border-b last:border-b-0", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[derive(Clone)]
pub struct AccordionItemContext {
    pub value: String,
}

#[component]
pub fn AccordionTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<Signal<AccordionContext>>();
    let item_ctx = use_context::<Signal<AccordionItemContext>>();
    let item_value = item_ctx.read().value.clone();
    let is_open = ctx.read().open_items.contains(&item_value);
    let item_value_clone = item_value.clone();

    let classes = cn(&[
        "flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none cursor-pointer",
        "hover:underline focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:pointer-events-none disabled:opacity-50",
        &class,
    ]);

    let chevron_rotation = if is_open { "rotate-180" } else { "" };

    rsx! {
        div {
            class: "flex",
            button {
                r#type: "button",
                class: "{classes}",
                "aria-expanded": if is_open { "true" } else { "false" },
                onclick: move |_| {
                    let mut new_items = ctx.read().open_items.clone();
                    let multiple = ctx.read().multiple;
                    let onchange = ctx.read().onchange.clone();
                    if is_open {
                        new_items.retain(|v| v != &item_value_clone);
                    } else {
                        if !multiple {
                            new_items.clear();
                        }
                        new_items.push(item_value_clone.clone());
                    }
                    ctx.write().open_items = new_items.clone();
                    if let Some(handler) = onchange {
                        handler.call(new_items);
                    }
                },
                {children}
                // Chevron icon
                svg {
                    class: "pointer-events-none size-4 shrink-0 translate-y-0.5 text-muted-foreground transition-transform duration-200 {chevron_rotation}",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path {
                        d: "m6 9 6 6 6-6",
                    }
                }
            }
        }
    }
}

#[component]
pub fn AccordionContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<AccordionContext>>();
    let item_ctx = use_context::<Signal<AccordionItemContext>>();
    let item_value = item_ctx.read().value.clone();
    let is_open = ctx.read().open_items.contains(&item_value);

    let grid_rows = if is_open { "grid-rows-[1fr]" } else { "grid-rows-[0fr]" };

    rsx! {
        div {
            class: "grid transition-[grid-template-rows] duration-200 ease-out {grid_rows}",
            div {
                class: "min-h-0 overflow-hidden",
                div {
                    class: cn(&["pb-4 pt-0 text-sm", &class]),
                    {children}
                }
            }
        }
    }
}
