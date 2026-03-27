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
    let mut ctx = use_context::<Signal<RadioGroupContext>>();
    let is_checked = ctx.read().value == value;

    let classes = cn(&[
        "aspect-square size-4 shrink-0 rounded-full border border-input text-primary shadow-xs transition-[color,box-shadow] outline-none cursor-pointer",
        "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:cursor-not-allowed disabled:opacity-50",
        "dark:bg-input/30",
        &class,
    ]);
    let value_clone = value.clone();
    let dot_class = if is_checked { "scale-100 opacity-100" } else { "scale-0 opacity-0" };

    rsx! {
        button {
            r#type: "button",
            role: "radio",
            class: "{classes}",
            id,
            disabled,
            "aria-checked": if is_checked { "true" } else { "false" },
            onclick: move |_| {
                let onchange = ctx.read().onchange.clone();
                ctx.write().value = value_clone.clone();
                if let Some(handler) = onchange {
                    handler.call(value_clone.clone());
                }
            },
            span {
                class: "flex items-center justify-center",
                svg {
                    class: "size-2 fill-primary transition-[transform,opacity] duration-150 ease-out {dot_class}",
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
