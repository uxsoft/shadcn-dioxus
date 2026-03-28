use dioxus::prelude::*;
use super::utils::cn;

const BASE: &str = "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50";

#[component]
pub fn Label(
    #[props(default)] r#for: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[BASE, &class]);
    rsx! {
        label {
            class: "{classes}",
            r#for,
            {children}
        }
    }
}
