use dioxus::prelude::*;
use super::utils::cn;

const BASE: &str = "h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none selection:bg-primary selection:text-primary-foreground file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-input/30 focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40";

#[component]
pub fn Input(
    #[props(default)] value: String,
    #[props(default)] r#type: Option<String>,
    #[props(default)] placeholder: String,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = cn(&[BASE, &class]);
    rsx! {
        input {
            r#type: r#type.unwrap_or_else(|| "text".into()),
            class: "{classes}",
            value,
            placeholder,
            disabled,
            id,
            oninput: move |evt| {
                if let Some(handler) = &oninput {
                    handler.call(evt);
                }
            },
        }
    }
}
