use dioxus::prelude::*;
use super::utils::cn;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AvatarSize {
    Sm,
    #[default]
    Default,
    Lg,
}

fn size_class(size: AvatarSize) -> &'static str {
    match size {
        AvatarSize::Sm => "size-6",
        AvatarSize::Default => "size-8",
        AvatarSize::Lg => "size-10",
    }
}

#[component]
pub fn Avatar(
    #[props(default)] size: AvatarSize,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "relative flex shrink-0 overflow-hidden rounded-full select-none",
        size_class(size),
        &class,
    ]);
    rsx! {
        span {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn AvatarImage(
    src: String,
    #[props(default)] alt: String,
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&["aspect-square size-full object-cover", &class]);
    rsx! {
        img {
            class: "{classes}",
            src,
            alt,
        }
    }
}

#[component]
pub fn AvatarFallback(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "flex size-full items-center justify-center rounded-full bg-muted text-sm text-muted-foreground",
        &class,
    ]);
    rsx! {
        span {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn AvatarGroup(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex -space-x-2", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}
