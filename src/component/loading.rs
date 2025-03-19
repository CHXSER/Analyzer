use std::{
    path::PathBuf,
    sync::{mpsc, Arc, Mutex},
};

use dioxus::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::model::{media::Media, photo::Photo, video::Video};

#[component]
pub fn Loading(folder_path: String) -> Element {
    let mut progress = use_signal(|| 0);
    let total_files = use_signal(|| 1);
    let completed_files = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    let folder_path_clone = folder_path.clone();
    spawn({
        let mut total_files = total_files.to_owned();
        let completed_files = completed_files.clone();
        let tx = tx.clone();

        async move {
            let files: Vec<PathBuf> = std::fs::read_dir(folder_path_clone)
                .unwrap()
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .filter(|path| {
                    path.is_file()
                        && path
                            .extension()
                            .map(|ext| {
                                matches!(
                                    ext.to_str(),
                                    Some("jpg" | "png" | "jpeg" | "gif" | "mp4" | "webm")
                                )
                            })
                            .unwrap_or(false)
                })
                .collect();

            total_files.set(files.len());

            let media_files: Vec<Media> = files
                .into_par_iter()
                .filter_map(|path| {
                    let media = if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        match ext {
                            "jpg" | "jpeg" | "png" => Photo::new(&path).map(Media::Photo),
                            "mp4" | "gif" | "webm" => Video::new(&path).map(Media::Video),
                            _ => None,
                        }
                    } else {
                        None
                    };

                    let mut count = completed_files.lock().unwrap();
                    *count += 1;
                    tx.send(*count).unwrap();

                    media
                })
                .collect();
        }
    });

    spawn(async move {
        while let Ok(value) = rx.recv() {
            progress.set(value);
        }
    });

    rsx! {
        div { id: "loading-container",
            h1 { "Analyzing {folder_path}" }
            div { id: "progress-bar",
                div {
                    style: "width: {progress() * 100 / total_files()}%;",
                    class: "progress-fill",
                }
            }
            p { "{progress()} / {total_files()} ({progress() * 100 / total_files()}%)" }
        }
    }
}
