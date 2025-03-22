use dioxus::prelude::*;

use crate::{Route, DELETE_QUEUE, DUPS};

#[component]
pub fn Summary() -> Element {
    rsx! {
        div { id: "container",
            h1 { "Summary" }
            p { "Confirm changes or go back" }
            div { id: "content",
                DeleteFiles {}
                IgnorePairs {}
                Buttons {}
            }
        }
    }
}

#[component]
fn DeleteFiles() -> Element {
    rsx! {
        div { class: "delete-container",
            p { "Files to delete: " }
            for path in DELETE_QUEUE().0 {
                p { "{path.to_str().unwrap()}" }
            }
        }
    }
}

#[component]
fn IgnorePairs() -> Element {
    rsx! {
        div { class: "ignore-container" }
    }
}

#[component]
fn Buttons() -> Element {
    rsx! {
        div { class: "confirm-container",
            button {
                id: "file-picker",
                onclick: move |_| {
                    for path in DELETE_QUEUE().0 {
                        std::fs::remove_file(path).unwrap();
                    }
                    DUPS.write().clear();
                    DELETE_QUEUE().0.clear();
                    use_navigator().push(Route::Home);
                },
                "CONFIRM"
            }
            button {
                id: "file-picker",
                onclick: move |_| {
                    DUPS.write().clear();
                    DELETE_QUEUE().0.clear();
                    use_navigator().push(Route::Home);
                },
                "CANCEL"
            }
        }
    }
}
