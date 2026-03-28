use dioxus::prelude::*;
use super::utils::cn;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

fn variant_class(variant: AlertVariant) -> &'static str {
    match variant {
        AlertVariant::Default => "bg-card text-card-foreground",
        AlertVariant::Destructive => "bg-card text-destructive *:data-[slot=alert-description]:text-destructive/90 [&>svg]:text-current",
    }
}

const BASE: &str = "relative grid w-full grid-cols-[0_1fr] items-start gap-y-0.5 rounded-lg border px-4 py-3 text-sm has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] has-[>svg]:gap-x-3 [&>svg]:size-4 [&>svg]:translate-y-0.5 [&>svg]:text-current";

#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[BASE, variant_class(variant), &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "alert",
            {children}
        }
    }
}

#[component]
pub fn AlertTitle(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight",
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
pub fn AlertDescription(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "col-start-2 grid justify-items-start gap-1 text-sm text-muted-foreground [&_p]:leading-relaxed",
        &class,
    ]);
    rsx! {
        div {
            "data-slot": "alert-description",
            class: "{classes}",
            {children}
        }
    }
}
