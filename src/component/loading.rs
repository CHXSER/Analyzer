use dioxus::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::PathBuf;
use tokio::sync::oneshot;
use vid_dup_finder_lib::{search, MatchGroup, VideoHash};

use crate::{
    model::{
        media::{DuplicateMedia, Media},
        photo::{self, find_similar_images, Photo},
        video::Video,
    },
    Route, DUPS, SETTINGS,
};

#[component]
pub fn Loading(folder_path: String) -> Element {
    let mut loading = use_signal(|| true);
    let image_tol = SETTINGS().image_tolerance();
    let video_tol = SETTINGS().video_tolerance();
    let folder_path_clone = folder_path.clone();
    if loading() && DUPS.is_empty() {
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
                                            "jpg" | "jpeg" | "png" | "gif" => {
                                                Photo::new(&path).map(Media::Photo)
                                            }
                                            "mp4" | "webm" => Video::new(&path).map(Media::Video),
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

                    let mut photos: Vec<Photo> = Vec::new();
                    let mut videos: Vec<Video> = Vec::new();

                    for media in media_files {
                        match media {
                            Media::Photo(a) => {
                                photos.push(a);
                            }
                            Media::Video(a) => {
                                videos.push(a);
                            }
                        }
                    }

                    let photo_duplicates: Vec<photo::PhotoMatchGroup> =
                        find_similar_images(&photos, image_tol);
                    let video_hashes: Vec<VideoHash> =
                        videos.iter().map(|vid| vid.hash.clone()).collect();
                    let video_duplicates: Vec<MatchGroup> = search(video_hashes, video_tol);

                    let mut dups: Vec<DuplicateMedia> = Vec::new();

                    for group in photo_duplicates {
                        dups.push(DuplicateMedia::PhotoMatchGroup(group));
                    }

                    for group in video_duplicates {
                        dups.push(DuplicateMedia::VideoMatchGroup(group));
                    }

                    let _ = tx.send(dups);
                });

                let a = rx.await.unwrap();
                *DUPS.write() = a;
                loading.set(false);
            }
        });
    } else {
        let _ = use_navigator().push(Route::Comparison {});
    }

    rsx! {
        div { id: "container",
            div { id: "loading-content",
                h2 { "Analyzing..." }
                p { id: "folder-path", "{folder_path}" }

                if loading() {
                    div { class: "progress-bar-container",
                        div { id: "progress-bar" }
                    }
                    p { "Processing files..." }
                } else {
                    p { "Analysis completed!" }
                }
            }
        }
    }
}
