use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NoDups() -> Element {
    rsx! {
        div { id: "container",
            h1 { "No duplicates found" }
            div { id: "content",
                p { "You're good to go!" }
                button {
                    id: "file-picker",
                    onclick: move |_| {
                        use_navigator().push(Route::Home);
                    },
                    "BACK"
                }
            }
        }
    }
}
