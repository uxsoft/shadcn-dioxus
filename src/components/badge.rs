use dioxus::prelude::*;
use super::utils::cn;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

fn variant_class(variant: BadgeVariant) -> &'static str {
    match variant {
        BadgeVariant::Default => "bg-primary text-primary-foreground",
        BadgeVariant::Secondary => "bg-secondary text-secondary-foreground",
        BadgeVariant::Destructive => "bg-destructive text-white dark:bg-destructive/60",
        BadgeVariant::Outline => "border-border text-foreground",
        BadgeVariant::Ghost => "",
        BadgeVariant::Link => "text-primary underline-offset-4",
    }
}

const BASE: &str = "inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden rounded-full border border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap transition-[color,box-shadow] [&>svg]:pointer-events-none [&>svg]:size-3";

#[component]
pub fn Badge(
    #[props(default)] variant: BadgeVariant,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[BASE, variant_class(variant), &class]);
    rsx! {
        span {
            class: "{classes}",
            {children}
        }
    }
}
