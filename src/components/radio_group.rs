use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn RadioGroup(
    #[props(default)] value: String,
    #[props(default)] class: String,
    onchange: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(RadioGroupContext {
        value: value.clone(),
        onchange: onchange.clone(),
    }));

    let classes = cn(&["grid gap-3", &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "radiogroup",
            {children}
        }
    }
}

#[derive(Clone)]
pub struct RadioGroupContext {
    pub value: String,
    pub onchange: Option<EventHandler<String>>,
}

#[component]
pub fn RadioGroupItem(
    value: String,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
) -> Element {
    let ctx = use_context::<Signal<RadioGroupContext>>();
    let ctx_read = ctx.read();
    let is_checked = ctx_read.value == value;

    let classes = cn(&[
        "aspect-square size-4 shrink-0 rounded-full border border-input text-primary shadow-xs transition-[color,box-shadow] outline-none cursor-pointer",
        "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:cursor-not-allowed disabled:opacity-50",
        "dark:bg-input/30",
        &class,
    ]);
    let onchange_handler = ctx_read.onchange.clone();
    let value_clone = value.clone();

    rsx! {
        button {
            r#type: "button",
            role: "radio",
            class: "{classes}",
            id,
            disabled,
            "aria-checked": if is_checked { "true" } else { "false" },
            onclick: move |_| {
                if let Some(handler) = &onchange_handler {
                    handler.call(value_clone.clone());
                }
            },
            if is_checked {
                span {
                    class: "flex items-center justify-center",
                    svg {
                        class: "size-2 fill-primary",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        circle {
                            cx: "12",
                            cy: "12",
                            r: "12",
                        }
                    }
                }
            }
        }
    }
}
