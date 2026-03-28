use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn InputGroup(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "flex items-center rounded-md border border-input shadow-xs focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50",
        "has-[input:disabled]:opacity-50 has-[input:disabled]:cursor-not-allowed",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn InputGroupAddon(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "flex items-center px-3 text-sm text-muted-foreground bg-muted border-r border-input first:rounded-l-md last:rounded-r-md last:border-r-0 last:border-l",
        "h-full",
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn InputGroupInput(
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] r#type: String,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let input_type = if r#type.is_empty() { "text" } else { &r#type };
    let classes = cn(&[
        "flex h-9 w-full bg-transparent px-3 py-1 text-sm outline-none file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground",
        "first:rounded-l-md last:rounded-r-md",
        &class,
    ]);
    rsx! {
        input {
            r#type: "{input_type}",
            class: "{classes}",
            value,
            placeholder,
            disabled,
            oninput: move |e| {
                if let Some(handler) = &oninput {
                    handler.call(e);
                }
            },
        }
    }
}
