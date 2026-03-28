use dioxus::prelude::*;

use components::*;
use views::{Home, Navbar, ShowcasePage};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/showcase/:name")]
        ShowcasePage { name: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Title { "shadcn-dioxus" }

        ThemeProvider {
            Router::<Route> {}
            Toaster {}
        }
    }
}
