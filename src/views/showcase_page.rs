use super::showcases::*;
use dioxus::prelude::*;

#[component]
pub fn ShowcasePage(name: String) -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 py-8 max-w-6xl",
            {match name.as_str() {
                "accordion" => rsx! { AccordionShowcase {} },
                "alert" => rsx! { AlertShowcase {} },
                "alert-dialog" => rsx! { AlertDialogShowcase {} },
                "aspect-ratio" => rsx! { AspectRatioShowcase {} },
                "avatar" => rsx! { AvatarShowcase {} },
                "badge" => rsx! { BadgeShowcase {} },
                "breadcrumb" => rsx! { BreadcrumbShowcase {} },
                "button" => rsx! { ButtonShowcase {} },
                "card" => rsx! { CardShowcase {} },
                "checkbox" => rsx! { CheckboxShowcase {} },
                "collapsible" => rsx! { CollapsibleShowcase {} },
                "combobox" => rsx! { ComboboxShowcase {} },
                "dialog" => rsx! { DialogShowcase {} },
                "dropdown-menu" => rsx! { DropdownMenuShowcase {} },
                "form" => rsx! { FormShowcase {} },
                "hover-card" => rsx! { HoverCardShowcase {} },
                "input" => rsx! { InputShowcase {} },
                "input-group" => rsx! { InputGroupShowcase {} },
                "kbd" => rsx! { KbdShowcase {} },
                "label" => rsx! { LabelShowcase {} },
                "popover" => rsx! { PopoverShowcase {} },
                "progress" => rsx! { ProgressShowcase {} },
                "radio-group" => rsx! { RadioGroupShowcase {} },
                "scroll-area" => rsx! { ScrollAreaShowcase {} },
                "select" => rsx! { SelectShowcase {} },
                "separator" => rsx! { SeparatorShowcase {} },
                "sheet" => rsx! { SheetShowcase {} },
                "skeleton" => rsx! { SkeletonShowcase {} },
                "slider" => rsx! { SliderShowcase {} },
                "spinner" => rsx! { SpinnerShowcase {} },
                "switch" => rsx! { SwitchShowcase {} },
                "table" => rsx! { TableShowcase {} },
                "tabs" => rsx! { TabsShowcase {} },
                "textarea" => rsx! { TextareaShowcase {} },
                "toast" => rsx! { ToastShowcase {} },
                "toggle" => rsx! { ToggleShowcase {} },
                "tooltip" => rsx! { TooltipShowcase {} },
                _ => rsx! {
                    div {
                        class: "text-center py-20",
                        h1 {
                            class: "text-2xl font-bold mb-2",
                            "Component not found"
                        }
                        p {
                            class: "text-muted-foreground",
                            "The showcase \"{name}\" does not exist."
                        }
                    }
                },
            }}
            div { class: "h-20" }
        }
    }
}
