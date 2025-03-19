use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { id: "container",
            h1 { "Welcome!" }
            button { id: "settings-button", "⚙" }
            div { id: "content",
                p { "Select a folder to analyze" } 
                button { id: "file-picker", "Choose Folder" }
            }
        }
    }
}