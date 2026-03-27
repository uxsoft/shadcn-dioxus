use crate::components::*;
use crate::components::utils::Side;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    // ---- Local state for interactive demos ----
    let mut checkbox_checked = use_signal(|| false);
    let mut switch_checked = use_signal(|| true);
    let mut slider_val = use_signal(|| 50.0_f64);
    let mut progress_val = use_signal(|| 60.0_f64);
    let mut input_val = use_signal(|| String::new());
    let mut textarea_val = use_signal(|| String::new());
    let mut tab_val = use_signal(|| "account".to_string());
    let mut accordion_val = use_signal(|| vec!["item-1".to_string()]);
    let mut radio_val = use_signal(|| "option-1".to_string());
    let mut dialog_open = use_signal(|| false);
    let mut sheet_open = use_signal(|| false);
    let mut alert_dialog_open = use_signal(|| false);
    let mut select_val = use_signal(|| String::new());
    let mut dropdown_open = use_signal(|| false);
    let mut collapsible_open = use_signal(|| false);
    let mut toggle_pressed = use_signal(|| false);
    let mut popover_open = use_signal(|| false);
    let mut combobox_val = use_signal(|| String::new());

    let mut toast_state = use_context::<Signal<ToastState>>();

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

            // ==================== BUTTONS ====================
            SectionTitle { title: "Button" }
            div {
                class: "flex flex-wrap gap-3 items-center",
                Button { "Default" }
                Button { variant: ButtonVariant::Secondary, "Secondary" }
                Button { variant: ButtonVariant::Destructive, "Destructive" }
                Button { variant: ButtonVariant::Outline, "Outline" }
                Button { variant: ButtonVariant::Ghost, "Ghost" }
                Button { variant: ButtonVariant::Link, "Link" }
            }
            div {
                class: "flex flex-wrap gap-3 items-center mt-4",
                Button { size: ButtonSize::Xs, "Extra Small" }
                Button { size: ButtonSize::Sm, "Small" }
                Button { size: ButtonSize::Default, "Default" }
                Button { size: ButtonSize::Lg, "Large" }
                Button { disabled: true, "Disabled" }
            }

            // ==================== BADGE ====================
            SectionTitle { title: "Badge" }
            div {
                class: "flex flex-wrap gap-3 items-center",
                Badge { "Default" }
                Badge { variant: BadgeVariant::Secondary, "Secondary" }
                Badge { variant: BadgeVariant::Destructive, "Destructive" }
                Badge { variant: BadgeVariant::Outline, "Outline" }
            }

            // ==================== INPUT ====================
            SectionTitle { title: "Input" }
            div {
                class: "grid gap-4 max-w-sm",
                Input {
                    value: input_val(),
                    placeholder: "Type something...",
                    oninput: move |e: FormEvent| *input_val.write() = e.value(),
                }
                Input {
                    disabled: true,
                    placeholder: "Disabled input",
                }
            }

            // ==================== TEXTAREA ====================
            SectionTitle { title: "Textarea" }
            div {
                class: "max-w-sm",
                Textarea {
                    value: textarea_val(),
                    placeholder: "Write your message...",
                    oninput: move |e: FormEvent| *textarea_val.write() = e.value(),
                }
            }

            // ==================== LABEL ====================
            SectionTitle { title: "Label" }
            div {
                class: "flex items-center gap-2",
                Checkbox {
                    checked: checkbox_checked(),
                    id: Some("terms".to_string()),
                    onchange: move |v| checkbox_checked.set(v),
                }
                Label {
                    r#for: "terms".to_string(),
                    "Accept terms and conditions"
                }
            }

            // ==================== CHECKBOX ====================
            SectionTitle { title: "Checkbox" }
            div {
                class: "flex items-center gap-4",
                Checkbox {
                    checked: checkbox_checked(),
                    onchange: move |v| checkbox_checked.set(v),
                }
                span { class: "text-sm", "Checked: {checkbox_checked}" }
            }

            // ==================== SWITCH ====================
            SectionTitle { title: "Switch" }
            div {
                class: "flex items-center gap-4",
                Switch {
                    checked: switch_checked(),
                    onchange: move |v| switch_checked.set(v),
                }
                span { class: "text-sm", "Enabled: {switch_checked}" }
            }

            // ==================== SLIDER ====================
            SectionTitle { title: "Slider" }
            div {
                class: "max-w-sm space-y-2",
                Slider {
                    value: slider_val(),
                    min: 0.0,
                    max: 100.0,
                    step: 1.0,
                    onchange: move |v| slider_val.set(v),
                }
                span { class: "text-sm text-muted-foreground", "Value: {slider_val}" }
            }

            // ==================== PROGRESS ====================
            SectionTitle { title: "Progress" }
            div {
                class: "max-w-sm space-y-2",
                Progress { value: progress_val() }
                div {
                    class: "flex gap-2",
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            let v = (progress_val() - 10.0).max(0.0);
                            progress_val.set(v);
                        },
                        "-10"
                    }
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            let v = (progress_val() + 10.0).min(100.0);
                            progress_val.set(v);
                        },
                        "+10"
                    }
                }
            }

            // ==================== AVATAR ====================
            SectionTitle { title: "Avatar" }
            div {
                class: "flex gap-4 items-center",
                Avatar {
                    AvatarFallback { "JD" }
                }
                Avatar {
                    size: AvatarSize::Sm,
                    AvatarFallback { "SM" }
                }
                Avatar {
                    size: AvatarSize::Lg,
                    AvatarFallback { "LG" }
                }
            }

            // ==================== SEPARATOR ====================
            SectionTitle { title: "Separator" }
            div {
                class: "space-y-1",
                h4 { class: "text-sm font-medium leading-none", "shadcn/dioxus" }
                p { class: "text-sm text-muted-foreground", "A component library for Dioxus." }
                Separator {}
                p { class: "text-sm text-muted-foreground", "Built with Tailwind CSS." }
            }

            // ==================== SKELETON ====================
            SectionTitle { title: "Skeleton" }
            div {
                class: "flex items-center gap-4",
                Skeleton { class: "h-12 w-12 rounded-full".to_string() }
                div {
                    class: "space-y-2",
                    Skeleton { class: "h-4 w-[250px]".to_string() }
                    Skeleton { class: "h-4 w-[200px]".to_string() }
                }
            }

            // ==================== SPINNER ====================
            SectionTitle { title: "Spinner" }
            div {
                class: "flex gap-4 items-center",
                Spinner { size: SpinnerSize::Sm }
                Spinner {}
                Spinner { size: SpinnerSize::Lg }
            }

            // ==================== KBD ====================
            SectionTitle { title: "Kbd" }
            div {
                class: "flex gap-2 items-center",
                Kbd { "⌘" }
                Kbd { "K" }
                span { class: "text-sm text-muted-foreground", "to open command palette" }
            }

            // ==================== CARD ====================
            SectionTitle { title: "Card" }
            div {
                class: "max-w-sm",
                Card {
                    CardHeader {
                        CardTitle { "Create project" }
                        CardDescription { "Deploy your project in one-click." }
                    }
                    CardContent {
                        p { class: "text-sm text-muted-foreground", "Project configuration goes here." }
                    }
                    CardFooter {
                        class: "justify-between",
                        Button { variant: ButtonVariant::Outline, "Cancel" }
                        Button { "Deploy" }
                    }
                }
            }

            // ==================== ALERT ====================
            SectionTitle { title: "Alert" }
            div {
                class: "space-y-4 max-w-lg",
                Alert {
                    AlertTitle { "Heads up!" }
                    AlertDescription { "You can add components to your app using the CLI." }
                }
                Alert {
                    variant: AlertVariant::Destructive,
                    AlertTitle { "Error" }
                    AlertDescription { "Your session has expired. Please log in again." }
                }
            }

            // ==================== TABS ====================
            SectionTitle { title: "Tabs" }
            div {
                class: "max-w-md",
                Tabs {
                    value: tab_val(),
                    onchange: move |v| tab_val.set(v),
                    TabsList {
                        TabsTrigger { value: "account", "Account" }
                        TabsTrigger { value: "password", "Password" }
                    }
                    TabsContent {
                        value: "account",
                        Card {
                            CardHeader {
                                CardTitle { "Account" }
                                CardDescription { "Make changes to your account here." }
                            }
                            CardContent {
                                p { class: "text-sm", "Account content goes here." }
                            }
                        }
                    }
                    TabsContent {
                        value: "password",
                        Card {
                            CardHeader {
                                CardTitle { "Password" }
                                CardDescription { "Change your password here." }
                            }
                            CardContent {
                                p { class: "text-sm", "Password content goes here." }
                            }
                        }
                    }
                }
            }

            // ==================== ACCORDION ====================
            SectionTitle { title: "Accordion" }
            div {
                class: "max-w-md",
                Accordion {
                    value: accordion_val(),
                    onchange: move |v| accordion_val.set(v),
                    AccordionItem {
                        value: "item-1",
                        AccordionTrigger { "Is it accessible?" }
                        AccordionContent { "Yes. It adheres to the WAI-ARIA design pattern." }
                    }
                    AccordionItem {
                        value: "item-2",
                        AccordionTrigger { "Is it styled?" }
                        AccordionContent { "Yes. It comes with default styles that match the other components' aesthetic." }
                    }
                    AccordionItem {
                        value: "item-3",
                        AccordionTrigger { "Is it animated?" }
                        AccordionContent { "Yes. It's animated by default, but you can disable it if you prefer." }
                    }
                }
            }

            // ==================== RADIO GROUP ====================
            SectionTitle { title: "Radio Group" }
            div {
                class: "max-w-sm",
                RadioGroup {
                    value: radio_val(),
                    onchange: move |v| radio_val.set(v),
                    div {
                        class: "flex items-center gap-2",
                        RadioGroupItem { value: "option-1", id: Some("r1".to_string()) }
                        Label { r#for: "r1".to_string(), "Default" }
                    }
                    div {
                        class: "flex items-center gap-2",
                        RadioGroupItem { value: "option-2", id: Some("r2".to_string()) }
                        Label { r#for: "r2".to_string(), "Comfortable" }
                    }
                    div {
                        class: "flex items-center gap-2",
                        RadioGroupItem { value: "option-3", id: Some("r3".to_string()) }
                        Label { r#for: "r3".to_string(), "Compact" }
                    }
                }
            }

            // ==================== TOGGLE ====================
            SectionTitle { title: "Toggle" }
            div {
                class: "flex gap-3 items-center",
                Toggle {
                    pressed: toggle_pressed(),
                    onchange: move |v| toggle_pressed.set(v),
                    svg {
                        class: "size-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                        path { d: "M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                    }
                }
                Toggle {
                    variant: ToggleVariant::Outline,
                    svg {
                        class: "size-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M4 7V4h16v3" }
                        path { d: "M9 20h6" }
                        path { d: "M12 4v16" }
                    }
                }
            }

            // ==================== COLLAPSIBLE ====================
            SectionTitle { title: "Collapsible" }
            div {
                class: "max-w-sm space-y-2",
                Collapsible {
                    open: collapsible_open(),
                    onchange: move |v| collapsible_open.set(v),
                    div {
                        class: "flex items-center justify-between",
                        h4 { class: "text-sm font-semibold", "3 items starred" }
                        CollapsibleTrigger {
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                if collapsible_open() { "Hide" } else { "Show" }
                            }
                        }
                    }
                    div {
                        class: "rounded-md border px-4 py-2 text-sm",
                        "Item 1"
                    }
                    CollapsibleContent {
                        class: "space-y-2 mt-2",
                        div {
                            class: "rounded-md border px-4 py-2 text-sm",
                            "Item 2"
                        }
                        div {
                            class: "rounded-md border px-4 py-2 text-sm",
                            "Item 3"
                        }
                    }
                }
            }

            // ==================== SELECT ====================
            SectionTitle { title: "Select" }
            div {
                class: "max-w-[200px]",
                Select {
                    value: select_val(),
                    onchange: move |v| select_val.set(v),
                    SelectTrigger {
                        placeholder: "Select a fruit".to_string(),
                        SelectValue { placeholder: "Select a fruit" }
                    }
                    SelectContent {
                        SelectGroup {
                            SelectLabel { "Fruits" }
                            SelectItem { value: "apple", "Apple" }
                            SelectItem { value: "banana", "Banana" }
                            SelectItem { value: "blueberry", "Blueberry" }
                            SelectItem { value: "grapes", "Grapes" }
                            SelectItem { value: "pineapple", "Pineapple" }
                        }
                    }
                }
            }

            // ==================== COMBOBOX ====================
            SectionTitle { title: "Combobox" }
            div {
                class: "max-w-[200px]",
                Combobox {
                    value: combobox_val(),
                    onchange: move |v| combobox_val.set(v),
                    ComboboxTrigger { placeholder: "Select framework..." }
                    ComboboxContent {
                        ComboboxItem { value: "next", label: "Next.js" }
                        ComboboxItem { value: "sveltekit", label: "SvelteKit" }
                        ComboboxItem { value: "nuxt", label: "Nuxt.js" }
                        ComboboxItem { value: "remix", label: "Remix" }
                        ComboboxItem { value: "astro", label: "Astro" }
                        ComboboxEmpty { "No framework found." }
                    }
                }
            }

            // ==================== DROPDOWN MENU ====================
            SectionTitle { title: "Dropdown Menu" }
            DropdownMenu {
                open: dropdown_open(),
                onclose: move |_| dropdown_open.set(false),
                DropdownMenuTrigger {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| dropdown_open.set(!dropdown_open()),
                        "Open Menu"
                    }
                }
                DropdownMenuContent {
                    DropdownMenuLabel { "My Account" }
                    DropdownMenuSeparator {}
                    DropdownMenuItem {
                        onclick: move |_| dropdown_open.set(false),
                        "Profile"
                    }
                    DropdownMenuItem {
                        onclick: move |_| dropdown_open.set(false),
                        "Billing"
                    }
                    DropdownMenuItem {
                        onclick: move |_| dropdown_open.set(false),
                        "Settings"
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuItem {
                        destructive: true,
                        onclick: move |_| dropdown_open.set(false),
                        "Log out"
                    }
                }
            }

            // ==================== POPOVER ====================
            SectionTitle { title: "Popover" }
            Popover {
                open: popover_open(),
                onclose: move |_| popover_open.set(false),
                PopoverTrigger {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| popover_open.set(!popover_open()),
                        "Open popover"
                    }
                }
                PopoverContent {
                    div {
                        class: "grid gap-4",
                        div {
                            class: "space-y-2",
                            h4 { class: "font-medium leading-none", "Dimensions" }
                            p { class: "text-sm text-muted-foreground", "Set the dimensions for the layer." }
                        }
                    }
                }
            }

            // ==================== TOOLTIP ====================
            SectionTitle { title: "Tooltip" }
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button { variant: ButtonVariant::Outline, "Hover me" }
                    }
                    TooltipContent {
                        "Add to library"
                    }
                }
            }

            // ==================== HOVER CARD ====================
            SectionTitle { title: "Hover Card" }
            HoverCard {
                HoverCardTrigger {
                    Button { variant: ButtonVariant::Link, "@dioxuslabs" }
                }
                HoverCardContent {
                    div {
                        class: "space-y-1",
                        h4 { class: "text-sm font-semibold", "@dioxuslabs" }
                        p { class: "text-sm text-muted-foreground",
                            "The Dioxus framework – fullstack, crossplatform, and blazingly fast."
                        }
                    }
                }
            }

            // ==================== DIALOG ====================
            SectionTitle { title: "Dialog" }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| dialog_open.set(true),
                "Open Dialog"
            }
            Dialog {
                open: dialog_open(),
                onclose: move |_| dialog_open.set(false),
                DialogContent {
                    DialogHeader {
                        DialogTitle { "Edit profile" }
                        DialogDescription { "Make changes to your profile here. Click save when you're done." }
                    }
                    div {
                        class: "grid gap-4 py-4",
                        Input { placeholder: "Name" }
                        Input { placeholder: "Username" }
                    }
                    DialogFooter {
                        Button {
                            onclick: move |_| dialog_open.set(false),
                            "Save changes"
                        }
                    }
                }
            }

            // ==================== SHEET ====================
            SectionTitle { title: "Sheet" }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| sheet_open.set(true),
                "Open Sheet"
            }
            Sheet {
                open: sheet_open(),
                side: Side::Right,
                onclose: move |_| sheet_open.set(false),
                SheetContent {
                    SheetHeader {
                        SheetTitle { "Edit profile" }
                        SheetDescription { "Make changes to your profile here." }
                    }
                    div {
                        class: "grid gap-4 py-4",
                        Input { placeholder: "Name" }
                        Input { placeholder: "Username" }
                    }
                    SheetFooter {
                        Button {
                            onclick: move |_| sheet_open.set(false),
                            "Save changes"
                        }
                    }
                }
            }

            // ==================== ALERT DIALOG ====================
            SectionTitle { title: "Alert Dialog" }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| alert_dialog_open.set(true),
                "Open Alert Dialog"
            }
            AlertDialog {
                open: alert_dialog_open(),
                onclose: move |_| alert_dialog_open.set(false),
                AlertDialogContent {
                    AlertDialogHeader {
                        AlertDialogTitle { "Are you absolutely sure?" }
                        AlertDialogDescription {
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { "Cancel" }
                        AlertDialogAction {
                            onclick: move |_| alert_dialog_open.set(false),
                            "Continue"
                        }
                    }
                }
            }

            // ==================== TOAST ====================
            SectionTitle { title: "Toast" }
            div {
                class: "flex gap-3",
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        toast_state.write().add("Event created", "Sunday, December 03, 2023 at 9:00 AM", ToastVariant::Default);
                    },
                    "Show Toast"
                }
                Button {
                    variant: ButtonVariant::Destructive,
                    onclick: move |_| {
                        toast_state.write().add("Error", "Something went wrong!", ToastVariant::Destructive);
                    },
                    "Show Error Toast"
                }
            }

            // ==================== TABLE ====================
            SectionTitle { title: "Table" }
            div {
                class: "max-w-2xl",
                Table {
                    TableHeader {
                        TableRow {
                            TableHead { "Invoice" }
                            TableHead { "Status" }
                            TableHead { "Method" }
                            TableHead { class: "text-right".to_string(), "Amount" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { class: "font-medium".to_string(), "INV001" }
                            TableCell { "Paid" }
                            TableCell { "Credit Card" }
                            TableCell { class: "text-right".to_string(), "$250.00" }
                        }
                        TableRow {
                            TableCell { class: "font-medium".to_string(), "INV002" }
                            TableCell { "Pending" }
                            TableCell { "PayPal" }
                            TableCell { class: "text-right".to_string(), "$150.00" }
                        }
                        TableRow {
                            TableCell { class: "font-medium".to_string(), "INV003" }
                            TableCell { "Unpaid" }
                            TableCell { "Bank Transfer" }
                            TableCell { class: "text-right".to_string(), "$350.00" }
                        }
                    }
                }
            }

            // ==================== BREADCRUMB ====================
            SectionTitle { title: "Breadcrumb" }
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "#", "Home" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbLink { href: "#", "Components" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbPage { "Breadcrumb" }
                    }
                }
            }

            // ==================== ASPECT RATIO ====================
            SectionTitle { title: "Aspect Ratio" }
            div {
                class: "max-w-[300px]",
                AspectRatio {
                    ratio: 16.0 / 9.0,
                    div {
                        class: "flex items-center justify-center w-full h-full bg-muted rounded-md text-sm text-muted-foreground",
                        "16:9"
                    }
                }
            }

            // ==================== FORM ====================
            SectionTitle { title: "Form" }
            div {
                class: "max-w-sm space-y-4",
                FormField {
                    Label { "Username" }
                    Input { placeholder: "Enter your username" }
                    FormDescription { "This is your public display name." }
                }
                FormField {
                    Label { "Email" }
                    Input { r#type: "email".to_string(), placeholder: "Enter your email" }
                    FormMessage { message: "Email is required." }
                }
            }

            // ==================== INPUT GROUP ====================
            SectionTitle { title: "Input Group" }
            div {
                class: "max-w-sm space-y-4",
                InputGroup {
                    InputGroupAddon { "https://" }
                    InputGroupInput { placeholder: "www.example.com" }
                }
                InputGroup {
                    InputGroupInput { placeholder: "0.00" }
                    InputGroupAddon { "USD" }
                }
            }

            // ==================== SCROLL AREA ====================
            SectionTitle { title: "Scroll Area" }
            ScrollArea {
                class: "h-[200px] w-[250px] rounded-md border p-4".to_string(),
                for i in 0..20 {
                    div {
                        class: "py-1 text-sm",
                        "Item {i}"
                    }
                    if i < 19 {
                        Separator {}
                    }
                }
            }

            // Spacer at the bottom
            div { class: "h-20" }
        }
    }
}

#[component]
fn SectionTitle(title: String) -> Element {
    rsx! {
        div {
            class: "mt-12 mb-4",
            h2 {
                class: "text-2xl font-semibold tracking-tight",
                "{title}"
            }
            Separator { class: "mt-2".to_string() }
        }
    }
}
