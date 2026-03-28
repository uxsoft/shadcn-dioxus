use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn ComboboxShowcase() -> Element {
    let mut combobox_val = use_signal(|| String::new());

    rsx! {
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
    }
}
