use dioxus::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::PathBuf;
use tokio::sync::oneshot;

use crate::{
    model::{media::Media, photo::Photo, video::Video},
    Route,
};

#[component]
pub fn Loading(folder_path: String) -> Element {
    let mut loading = use_signal(|| true);

    let folder_path_clone = folder_path.clone();
    if loading() {
        spawn({
            async move {
                let (tx, rx) = oneshot::channel();

                tokio::spawn(async move {
                    let media_files: Vec<Media> = tokio::task::spawn_blocking(move || {
                        let files: Vec<PathBuf> = std::fs::read_dir(&folder_path_clone)
                            .unwrap()
                            .filter_map(|entry| entry.ok().map(|e| e.path()))
                            .filter(|path| {
                                path.is_file()
                                    && path
                                        .extension()
                                        .map(|ext| {
                                            matches!(
                                                ext.to_str(),
                                                Some(
                                                    "jpg" | "png" | "jpeg" | "gif" | "mp4" | "webm"
                                                )
                                            )
                                        })
                                        .unwrap_or(false)
                            })
                            .collect();

                        files
                            .into_par_iter()
                            .filter_map(|path| {
                                let media =
                                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                        match ext {
                                            "jpg" | "jpeg" | "png" => {
                                                Photo::new(&path).map(Media::Photo)
                                            }
                                            "mp4" | "gif" | "webm" => {
                                                Video::new(&path).map(Media::Video)
                                            }
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    };
                                media
                            })
                            .collect::<Vec<Media>>()
                    })
                    .await
                    .unwrap();

                    let _ = tx.send(media_files);
                });

                let _ = rx.await;
                loading.set(false);
            }
        });
    } else {
        let _ = use_navigator().push(Route::Home);
    }

    rsx! {
        div { class: "loading-container",
            div { class: "loading-content",
                h2 { "Analyzing..." }
                p { class: "folder-path", "{folder_path}" }

                if loading() {
                    div { class: "progress-bar-container",
                        div { class: "progress-bar" }
                    }
                    p { "Processing files..." }
                } else {
                    p { "Analysis completed!" }
                }
            }
        }
    }
}
