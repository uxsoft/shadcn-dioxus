use dioxus::prelude::*;
use crate::components::*;
use super::section_title::SectionTitle;

#[component]
pub fn TableShowcase() -> Element {
    rsx! {
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
    }
}
