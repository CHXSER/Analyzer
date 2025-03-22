use dioxus::prelude::*;

use crate::DELETE_QUEUE;

#[component]
pub fn Summary() -> Element {
    println!("{:?}", DELETE_QUEUE);
    rsx! {}
}
