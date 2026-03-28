use dioxus::prelude::*;
use super::utils::cn;

#[derive(PartialEq, Clone)]
pub struct ToastData {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub variant: ToastVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Destructive,
}

#[derive(Clone)]
pub struct ToastState {
    pub toasts: Vec<ToastData>,
    pub next_id: u32,
}

impl ToastState {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            next_id: 0,
        }
    }
    pub fn add(&mut self, title: impl Into<String>, description: impl Into<String>, variant: ToastVariant) {
        let id = self.next_id;
        self.next_id += 1;
        self.toasts.push(ToastData {
            id,
            title: title.into(),
            description: description.into(),
            variant,
        });
    }
    pub fn remove(&mut self, id: u32) {
        self.toasts.retain(|t| t.id != id);
    }
}

pub fn use_toast() -> Signal<ToastState> {
    use_context::<Signal<ToastState>>()
}

#[component]
pub fn Toaster(
    #[props(default)] class: String,
) -> Element {
    let mut toast_state = use_context::<Signal<ToastState>>();
    let toasts = toast_state.read().toasts.clone();

    if toasts.is_empty() {
        return rsx! {};
    }

    let classes = cn(&[
        "fixed bottom-0 right-0 z-[100] flex max-h-screen w-full flex-col-reverse gap-2 p-4 sm:max-w-[420px]",
        &class,
    ]);

    rsx! {
        div {
            class: "{classes}",
            for toast in toasts {
                Toast {
                    key: "{toast.id}",
                    toast_data: toast.clone(),
                }
            }
        }
    }
}

#[component]
fn Toast(toast_data: ToastData) -> Element {
    let mut toast_state = use_context::<Signal<ToastState>>();
    let id = toast_data.id;

    let variant_class = match toast_data.variant {
        ToastVariant::Default => "border bg-background text-foreground",
        ToastVariant::Destructive => "border-destructive bg-destructive text-white",
    };

    let classes = cn(&[
        "group pointer-events-auto relative flex w-full items-center justify-between gap-4 overflow-hidden rounded-md border p-4 shadow-lg transition-all",
        "animate-in slide-in-from-bottom fade-in-0",
        variant_class,
    ]);

    rsx! {
        div {
            class: "{classes}",
            div {
                class: "grid gap-1",
                if !toast_data.title.is_empty() {
                    div {
                        class: "text-sm font-semibold",
                        "{toast_data.title}"
                    }
                }
                if !toast_data.description.is_empty() {
                    div {
                        class: "text-sm opacity-90",
                        "{toast_data.description}"
                    }
                }
            }
            button {
                r#type: "button",
                class: "absolute top-1 right-1 rounded-md p-1 text-foreground/50 opacity-0 transition-opacity cursor-pointer group-hover:opacity-100 hover:text-foreground",
                onclick: move |_| {
                    toast_state.write().remove(id);
                },
                svg {
                    class: "size-4",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { d: "M18 6 6 18" }
                    path { d: "m6 6 12 12" }
                }
            }
        }
    }
}
