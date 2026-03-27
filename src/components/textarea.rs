use dioxus::prelude::*;
use super::utils::cn;

const BASE: &str = "flex field-sizing-content min-h-16 w-full rounded-md border border-input bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 md:text-sm dark:bg-input/30 dark:aria-invalid:ring-destructive/40";

#[component]
pub fn Textarea(
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] rows: Option<i64>,
    #[props(default)] id: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = cn(&[BASE, &class]);
    rsx! {
        textarea {
            class: "{classes}",
            value,
            placeholder,
            disabled,
            rows,
            id,
            oninput: move |evt| {
                if let Some(handler) = &oninput {
                    handler.call(evt);
                }
            },
        }
    }
}
