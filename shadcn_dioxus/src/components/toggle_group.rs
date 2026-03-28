use dioxus::prelude::*;
use super::utils::cn;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToggleGroupType {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone)]
pub struct ToggleGroupContext {
    pub values: Vec<String>,
    pub group_type: ToggleGroupType,
    pub onchange: Option<EventHandler<Vec<String>>>,
}

#[component]
pub fn ToggleGroup(
    #[props(default)] values: Vec<String>,
    #[props(default)] group_type: ToggleGroupType,
    #[props(default)] class: String,
    onchange: Option<EventHandler<Vec<String>>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(ToggleGroupContext {
        values: values.clone(),
        group_type,
        onchange: onchange.clone(),
    }));

    let classes = cn(&["flex items-center gap-1", &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "group",
            {children}
        }
    }
}

#[component]
pub fn ToggleGroupItem(
    value: String,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<ToggleGroupContext>>();
    let ctx_read = ctx.read();
    let is_active = ctx_read.values.contains(&value);

    let state_class = if is_active { "bg-accent text-accent-foreground" } else { "" };
    let classes = cn(&[
        "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-[color,box-shadow] outline-none cursor-pointer hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 h-9 min-w-9 px-2 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        state_class,
        &class,
    ]);

    let onchange_handler = ctx_read.onchange.clone();
    let group_type = ctx_read.group_type;
    let current_values = ctx_read.values.clone();
    let value_clone = value.clone();

    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            disabled,
            "aria-pressed": if is_active { "true" } else { "false" },
            onclick: move |_| {
                if let Some(handler) = &onchange_handler {
                    let mut new_values = current_values.clone();
                    match group_type {
                        ToggleGroupType::Single => {
                            new_values = if is_active {
                                vec![]
                            } else {
                                vec![value_clone.clone()]
                            };
                        }
                        ToggleGroupType::Multiple => {
                            if is_active {
                                new_values.retain(|v| v != &value_clone);
                            } else {
                                new_values.push(value_clone.clone());
                            }
                        }
                    }
                    handler.call(new_values);
                }
            },
            {children}
        }
    }
}
