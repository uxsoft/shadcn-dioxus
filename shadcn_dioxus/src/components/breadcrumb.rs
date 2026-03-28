use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Breadcrumb(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        nav {
            class: "{class}",
            "aria-label": "breadcrumb",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbList(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground break-words sm:gap-2.5",
        &class,
    ]);
    rsx! {
        ol {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbItem(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["inline-flex items-center gap-1.5", &class]);
    rsx! {
        li {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbLink(
    #[props(default)] href: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["transition-colors hover:text-foreground", &class]);
    rsx! {
        a {
            class: "{classes}",
            href,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbPage(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["font-normal text-foreground", &class]);
    rsx! {
        span {
            class: "{classes}",
            role: "link",
            "aria-disabled": "true",
            "aria-current": "page",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[props(default)] class: String,
) -> Element {
    let classes = cn(&["[&>svg]:size-3.5", &class]);
    rsx! {
        li {
            class: "{classes}",
            role: "presentation",
            "aria-hidden": "true",
            // Chevron right icon
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "size-3.5",
                path {
                    d: "m9 18 6-6-6-6",
                }
            }
        }
    }
}
