use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Carousel(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_signal(|| CarouselContext {
        current: 0,
        total: 0,
    });
    use_context_provider(|| ctx);

    let classes = cn(&["relative", &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "region",
            "aria-roledescription": "carousel",
            {children}
        }
    }
}

#[derive(Clone, Copy)]
pub struct CarouselContext {
    pub current: usize,
    pub total: usize,
}

#[component]
pub fn CarouselContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["flex overflow-hidden", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn CarouselItem(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["min-w-0 shrink-0 grow-0 basis-full", &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "group",
            "aria-roledescription": "slide",
            {children}
        }
    }
}

#[component]
pub fn CarouselPrevious(
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let classes = cn(&[
        "absolute left-4 top-1/2 -translate-y-1/2 z-10 inline-flex size-8 items-center justify-center rounded-full border bg-background shadow-sm cursor-pointer hover:bg-accent hover:text-accent-foreground disabled:opacity-50",
        &class,
    ]);
    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            svg {
                class: "size-4",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "m15 18-6-6 6-6" }
            }
            span { class: "sr-only", "Previous slide" }
        }
    }
}

#[component]
pub fn CarouselNext(
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let classes = cn(&[
        "absolute right-4 top-1/2 -translate-y-1/2 z-10 inline-flex size-8 items-center justify-center rounded-full border bg-background shadow-sm cursor-pointer hover:bg-accent hover:text-accent-foreground disabled:opacity-50",
        &class,
    ]);
    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            svg {
                class: "size-4",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "m9 18 6-6-6-6" }
            }
            span { class: "sr-only", "Next slide" }
        }
    }
}
