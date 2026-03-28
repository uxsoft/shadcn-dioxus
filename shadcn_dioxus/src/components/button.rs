use dioxus::prelude::*;
use super::utils::cn;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Xs,
    Sm,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

fn variant_class(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Destructive => "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:bg-destructive/60 dark:focus-visible:ring-destructive/40",
        ButtonVariant::Outline => "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    }
}

fn size_class(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "h-9 px-4 py-2",
        ButtonSize::Xs => "h-6 gap-1 rounded-md px-2 text-xs",
        ButtonSize::Sm => "h-8 gap-1.5 rounded-md px-3",
        ButtonSize::Lg => "h-10 rounded-md px-6",
        ButtonSize::Icon => "size-9",
        ButtonSize::IconXs => "size-6 rounded-md",
        ButtonSize::IconSm => "size-8",
        ButtonSize::IconLg => "size-10",
    }
}

const BASE: &str = "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none cursor-pointer focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 active:scale-[0.97] active:brightness-90 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] r#type: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let classes = cn(&[BASE, variant_class(variant), size_class(size), &class]);
    rsx! {
        button {
            r#type: r#type.unwrap_or_else(|| "button".into()),
            class: "{classes}",
            disabled,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}
