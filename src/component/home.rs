use dioxus::prelude::*;
use directories::UserDirs;
use rfd::AsyncFileDialog;

use crate::Route;

#[component]
pub fn Home() -> Element {
    let folder = use_signal(|| "".to_string());
    let pick = move |_evt| {
        let mut folder = folder.to_owned();
        spawn(async move {
            match pick_dir().await {
                Some(a) => {
                    folder.set(a.clone());
                    let _ = use_navigator().push(Route::Loading { folder_path: a });
                }
                None => folder.set("".to_string()),
            }
        });
    };

    rsx! {
        div { id: "container",
            h1 { "Welcome!" }
            button { id: "settings-button", "⚙" }
            div { id: "content",
                p { "Select a folder to analyze" }
                button { onclick: pick, id: "file-picker", "Choose Folder" }
                p { "{folder()}" }
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
