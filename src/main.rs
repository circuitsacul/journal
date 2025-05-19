mod components;
mod pages;

use pages::*;

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

static MOUSEDOWN: GlobalSignal<bool> = Signal::global(|| false);

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            id: "app-main",
            class: "h-screen overflow-hidden",
            onmousedown: move |_e| *MOUSEDOWN.write() = true,
            onmouseup: move |_e| *MOUSEDOWN.write() = false,
            onmouseleave: move |_e| *MOUSEDOWN.write() = false,

            Router::<Route> {}
        }
    }
}
