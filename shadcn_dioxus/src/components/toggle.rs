use super::utils::cn;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToggleSize {
    Sm,
    #[default]
    Default,
    Lg,
}

fn variant_class(variant: ToggleVariant) -> &'static str {
    match variant {
        ToggleVariant::Default => "bg-transparent",
        ToggleVariant::Outline => {
            "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground"
        }
    }
}

fn size_class(size: ToggleSize) -> &'static str {
    match size {
        ToggleSize::Sm => "h-8 min-w-8 px-1.5",
        ToggleSize::Default => "h-9 min-w-9 px-2",
        ToggleSize::Lg => "h-10 min-w-10 px-2.5",
    }
}

const BASE: &str = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none cursor-pointer hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 active:scale-[0.97] active:brightness-90 aria-pressed:bg-muted aria-pressed:brightness-90";

#[component]
pub fn Toggle(
    #[props(default)] pressed: ReadSignal<bool>,
    #[props(default)] variant: ToggleVariant,
    #[props(default)] size: ToggleSize,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onchange: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let classes = cn(&[BASE, variant_class(variant), size_class(size), &class]);
    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            disabled,
            "aria-pressed": if pressed() { "true" } else { "false" },
            "data-state": if pressed() { "on" } else { "off" },
            onclick: move |_| {
                if let Some(handler) = onchange {
                    handler.call(!pressed());
                }
            },
            {children}
        }
    }
}
