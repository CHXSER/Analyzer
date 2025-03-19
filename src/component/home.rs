use dioxus::prelude::*;
use directories::UserDirs;
use rfd::AsyncFileDialog;

use crate::HomeContext;

#[component]
pub fn Home() -> Element {
    let pick = async move |_evt| match pick_dir().await {
        Some(a) => {
            consume_context::<HomeContext>().path.set(a);
        }
        None => consume_context::<HomeContext>().path.set("".to_string()),
    };

    //let folder_path = use_context::<HomeContext>();

    rsx! {
        div { id: "container",
            h1 { "Welcome!" }
            button { id: "settings-button", "⚙" }
            div { id: "content",
                p { "Select a folder to analyze" }
                button { onclick: pick, id: "file-picker", "Choose Folder" }
                        //p { "{folder_path.path}" }
            }
        }
    }
}

async fn pick_dir() -> Option<String> {
    let user_dir = UserDirs::new().unwrap();
    let folder = AsyncFileDialog::new()
        .set_title("Folder to analyze")
        .set_directory(user_dir.picture_dir().unwrap())
        .pick_folder()
        .await;
    folder.map(|path| String::from(path.path().to_str().unwrap()))
}
