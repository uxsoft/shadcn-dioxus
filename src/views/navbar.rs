use crate::Route;
use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut theme = use_context::<Signal<ThemeState>>();
    let is_dark = theme.read().dark;

    rsx! {
        header {
            class: "sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur",
            div {
                class: "container flex h-14 items-center justify-between px-4 mx-auto",
                div {
                    class: "flex items-center gap-4",
                    Link {
                        to: Route::Home {},
                        class: "text-lg font-bold",
                        "shadcn/dioxus"
                    }
                }
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
        Outlet::<Route> {}
    }
}
