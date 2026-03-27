use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn FormField(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["space-y-2", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn FormMessage(
    #[props(default)] message: String,
    #[props(default)] class: String,
) -> Element {
    if message.is_empty() {
        return rsx! {};
    }
    let classes = cn(&["text-[0.8rem] font-medium text-destructive", &class]);
    rsx! {
        p {
            class: "{classes}",
            "{message}"
        }
    }
}

#[component]
pub fn FormDescription(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["text-[0.8rem] text-muted-foreground", &class]);
    rsx! {
        p {
            class: "{classes}",
            {children}
        }
    }
}
