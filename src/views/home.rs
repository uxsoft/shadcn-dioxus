use dioxus::prelude::*;
use super::showcases::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4 py-8 max-w-6xl",

            // Hero
            div {
                class: "mb-12 text-center",
                h1 {
                    class: "text-4xl font-bold tracking-tight mb-4",
                    "shadcn/dioxus"
                }
                p {
                    class: "text-xl text-muted-foreground max-w-2xl mx-auto",
                    "A port of shadcn/ui components for Dioxus. Beautiful, accessible, and customizable."
                }
            }

            ButtonShowcase {}
            BadgeShowcase {}
            InputShowcase {}
            TextareaShowcase {}
            LabelShowcase {}
            CheckboxShowcase {}
            SwitchShowcase {}
            SliderShowcase {}
            ProgressShowcase {}
            AvatarShowcase {}
            SeparatorShowcase {}
            SkeletonShowcase {}
            SpinnerShowcase {}
            KbdShowcase {}
            CardShowcase {}
            AlertShowcase {}
            TabsShowcase {}
            AccordionShowcase {}
            RadioGroupShowcase {}
            ToggleShowcase {}
            CollapsibleShowcase {}
            SelectShowcase {}
            ComboboxShowcase {}
            DropdownMenuShowcase {}
            PopoverShowcase {}
            TooltipShowcase {}
            HoverCardShowcase {}
            DialogShowcase {}
            SheetShowcase {}
            AlertDialogShowcase {}
            ToastShowcase {}
            TableShowcase {}
            BreadcrumbShowcase {}
            AspectRatioShowcase {}
            FormShowcase {}
            InputGroupShowcase {}
            ScrollAreaShowcase {}

            // Spacer at the bottom
            div { class: "h-20" }
        }
    }
}
