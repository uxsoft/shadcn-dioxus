use dioxus::prelude::*;
use super::utils::cn;

#[component]
pub fn Table(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["w-full caption-bottom text-sm", &class]);
    rsx! {
        div {
            class: "relative w-full overflow-auto",
            table {
                class: "{classes}",
                {children}
            }
        }
    }
}

#[component]
pub fn TableHeader(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["[&_tr]:border-b", &class]);
    rsx! {
        thead {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn TableBody(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["[&_tr:last-child]:border-0", &class]);
    rsx! {
        tbody {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn TableFooter(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["border-t bg-muted/50 font-medium [&>tr]:last:border-b-0", &class]);
    rsx! {
        tfoot {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn TableRow(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
        &class,
    ]);
    rsx! {
        tr {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn TableHead(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
        &class,
    ]);
    rsx! {
        th {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn TableCell(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&[
        "p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
        &class,
    ]);
    rsx! {
        td {
            class: "{classes}",
            {children}
        }
    }
}

#[component]
pub fn TableCaption(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let classes = cn(&["mt-4 text-sm text-muted-foreground", &class]);
    rsx! {
        caption {
            class: "{classes}",
            {children}
        }
    }
}
