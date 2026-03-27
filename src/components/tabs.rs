use dioxus::prelude::*;
use super::utils::cn;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TabsVariant {
    #[default]
    Default,
    Line,
}

#[derive(Clone)]
pub struct TabsContext {
    pub active: String,
    pub variant: TabsVariant,
    pub onchange: Option<EventHandler<String>>,
}

#[component]
pub fn Tabs(
    #[props(default)] value: String,
    #[props(default)] variant: TabsVariant,
    #[props(default)] class: String,
    onchange: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    use_context_provider(|| Signal::new(TabsContext {
        active: value.clone(),
        variant,
        onchange: onchange.clone(),
    }));

    let classes = cn(&["flex flex-col gap-2", &class]);
    rsx! {
        div {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn TabsList(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<TabsContext>>();
    let variant = ctx.read().variant;
    let variant_class = match variant {
        TabsVariant::Default => "bg-muted",
        TabsVariant::Line => "gap-1 bg-transparent",
    };
    let classes = cn(&[
        "inline-flex w-fit items-center justify-center rounded-lg p-[3px] text-muted-foreground h-9",
        variant_class,
        &class,
    ]);
    rsx! {
        div {
            class: "{classes}",
            role: "tablist",
            {children}
        }
    }
}

#[component]
pub fn TabsTrigger(
    value: String,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<TabsContext>>();
    let ctx_read = ctx.read();
    let is_active = ctx_read.active == value;
    let onchange_handler = ctx_read.onchange.clone();
    let value_clone = value.clone();

    let active_class = if is_active {
        "bg-background text-foreground shadow-sm dark:border-input dark:bg-input/30 dark:text-foreground"
    } else {
        ""
    };

    let classes = cn(&[
        "relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap text-foreground/60 transition-all cursor-pointer",
        "hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:pointer-events-none disabled:opacity-50",
        active_class,
        &class,
    ]);

    rsx! {
        button {
            r#type: "button",
            role: "tab",
            class: "{classes}",
            disabled,
            "aria-selected": if is_active { "true" } else { "false" },
            "data-state": if is_active { "active" } else { "inactive" },
            onclick: move |_| {
                if let Some(handler) = &onchange_handler {
                    handler.call(value_clone.clone());
                }
            },
            {children}
        }
    }
}

#[component]
pub fn TabsContent(
    value: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<Signal<TabsContext>>();
    let is_active = ctx.read().active == value;

    if !is_active {
        return rsx! {};
    }

    let classes = cn(&["flex-1 outline-none", &class]);
    rsx! {
        div {
            class: "{classes}",
            role: "tabpanel",
            {children}
        }
    }
}
