use crate::components::*;
use crate::Route;
use dioxus::prelude::*;

const SIDEBAR_GROUPS: &[(&str, &[(&str, &str)])] = &[
    (
        "Primitives",
        &[
            ("Button", "button"),
            ("Input", "input"),
            ("Textarea", "textarea"),
            ("Label", "label"),
            ("Separator", "separator"),
            ("Badge", "badge"),
            ("Skeleton", "skeleton"),
            ("Spinner", "spinner"),
            ("Kbd", "kbd"),
            ("Aspect Ratio", "aspect-ratio"),
            ("Progress", "progress"),
            ("Slider", "slider"),
            ("Avatar", "avatar"),
        ],
    ),
    (
        "Compound",
        &[
            ("Card", "card"),
            ("Alert", "alert"),
            ("Checkbox", "checkbox"),
            ("Switch", "switch"),
            ("Radio Group", "radio-group"),
            ("Toggle", "toggle"),
            ("Tabs", "tabs"),
            ("Accordion", "accordion"),
            ("Collapsible", "collapsible"),
            ("Table", "table"),
            ("Breadcrumb", "breadcrumb"),
            ("Scroll Area", "scroll-area"),
        ],
    ),
    (
        "Overlay",
        &[
            ("Dialog", "dialog"),
            ("Sheet", "sheet"),
            ("Alert Dialog", "alert-dialog"),
            ("Dropdown Menu", "dropdown-menu"),
            ("Select", "select"),
            ("Popover", "popover"),
            ("Tooltip", "tooltip"),
            ("Hover Card", "hover-card"),
        ],
    ),
    (
        "Complex",
        &[
            ("Combobox", "combobox"),
            ("Toast", "toast"),
            ("Form", "form"),
            ("Input Group", "input-group"),
        ],
    ),
];

#[component]
pub fn Navbar() -> Element {
    let mut theme = use_context::<Signal<ThemeState>>();
    let is_dark = theme.read().dark;

    let route: Route = use_route();
    let active_name = match &route {
        Route::ShowcasePage { name } => Some(name.clone()),
        _ => None,
    };

    rsx! {
        div {
            class: "flex min-h-screen w-full",

            // Sidebar
            aside {
                class: "sticky top-0 h-screen w-64 shrink-0 border-r bg-sidebar text-sidebar-foreground overflow-y-auto",
                div {
                    class: "flex flex-col gap-1 p-4",
                    Link {
                        to: Route::Home {},
                        class: "text-lg font-bold mb-2 hover:text-foreground/80",
                        "shadcn/dioxus"
                    }
                    for (group_label, items) in SIDEBAR_GROUPS.iter() {
                        div {
                            class: "mt-4",
                            div {
                                class: "flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium text-sidebar-foreground/70",
                                "{group_label}"
                            }
                            ul {
                                class: "flex w-full min-w-0 flex-col gap-0.5",
                                for (label, slug) in items.iter() {
                                    li {
                                        Link {
                                            to: Route::ShowcasePage { name: slug.to_string() },
                                            class: {
                                                let is_active = active_name.as_deref() == Some(*slug);
                                                let base = "flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm outline-none transition-colors cursor-pointer hover:bg-sidebar-accent hover:text-sidebar-accent-foreground";
                                                if is_active {
                                                    format!("{base} bg-sidebar-accent text-sidebar-accent-foreground font-medium")
                                                } else {
                                                    base.to_string()
                                                }
                                            },
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // bottom spacer
                    div { class: "h-4" }
                }
            }

            // Main content area
            div {
                class: "flex flex-1 flex-col",

                // Top header bar
                header {
                    class: "sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur",
                    div {
                        class: "flex h-14 items-center justify-end px-4",
                        div {
                            class: "flex items-center gap-2",
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Icon,
                                onclick: move |_| {
                                    theme.write().toggle_dark();
                                },
                                if is_dark {
                                    // Sun icon
                                    svg {
                                        class: "size-5",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        circle { cx: "12", cy: "12", r: "4" }
                                        path { d: "M12 2v2" }
                                        path { d: "M12 20v2" }
                                        path { d: "m4.93 4.93 1.41 1.41" }
                                        path { d: "m17.66 17.66 1.41 1.41" }
                                        path { d: "M2 12h2" }
                                        path { d: "M20 12h2" }
                                        path { d: "m6.34 17.66-1.41 1.41" }
                                        path { d: "m19.07 4.93-1.41 1.41" }
                                    }
                                } else {
                                    // Moon icon
                                    svg {
                                        class: "size-5",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Page content
                Outlet::<Route> {}
            }
        }
    }
}
