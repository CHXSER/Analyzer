use dioxus::prelude::*;
use humansize::{format_size, DECIMAL};

use crate::{DeleteQueue, Route, DELETE_QUEUE, DUPS};

#[component]
pub fn Summary() -> Element {
    let mut delete_size: Signal<u64> = use_signal(|| 0);
    use_effect(move || {
        let mut size: u64 = 0;
        for file in DELETE_QUEUE().0 {
            let metadata = std::fs::metadata(&file).unwrap();
            size += metadata.len();
        }

        delete_size.set(size);
    });

    rsx! {
        div { id: "container",
            h1 { "Summary" }
            p { "Confirm changes or go back" }
            div { id: "content",
                DeleteFiles { size: delete_size() }
                IgnorePairs {}
                Buttons {}
            }
        }
    }
}

#[component]
fn DeleteFiles(size: u64) -> Element {
    let real_size = format_size(size, DECIMAL);
    rsx! {
        div { class: "delete-container",
            p { "Number of files to delete: {DELETE_QUEUE().0.len()}" }
            p { id: "delete-file-size", "{real_size} will be deleted" }
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
                    *DUPS.write() = vec![];
                    *DELETE_QUEUE.write() = DeleteQueue(vec![]);
                    use_navigator().push(Route::Home);
                },
                "CONFIRM"
            }
            button {
                id: "file-picker",
                onclick: move |_| {
                    *DUPS.write() = vec![];
                    *DELETE_QUEUE.write() = DeleteQueue(vec![]);
                    use_navigator().push(Route::Home);
                },
                "CANCEL"
            }
        }
    }
}
