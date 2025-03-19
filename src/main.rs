use dioxus::prelude::*;

mod component;

use component::home::Home;

static MAIN_CSS: Asset = asset!("/assets/main.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");



fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Home {  }
        //Router::<Route> {}
    }
}
