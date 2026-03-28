use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Kbd(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "pointer-events-none inline-flex h-5 items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground select-none",
        &class,
    ]);
    rsx! {
        kbd {
            class: "{classes}",
            {children}
        }
    }
}
