use std::os::windows::fs::MetadataExt;

use dioxus::prelude::*;
use humansize::{format_size, DECIMAL};
use urlencoding::encode;

use crate::{model::media::DuplicateMedia, Route, DELETE_QUEUE, DUPS};

#[component]
pub fn Comparison() -> Element {
    if DUPS().is_empty() {
        use_navigator().push(Route::NoDups);
        return rsx! {};
    }

    let mut current_index: Signal<usize> = use_signal(|| 0);

    rsx! {
        div { id: "duplicate-container",
            div { class: "nav-bar",
                button { class: "nav-button prev", onclick: move |_| {
                    if current_index() >= 1 {
                        current_index.set(current_index - 1);
                    }
                }, "PREVIOUS" }
                div { class: "counter", "{current_index() + 1} of {DUPS().len()} duplicates" }
                button { class: "nav-button next", onclick: move |_| {
                    if current_index() <= DUPS().len() {
                        current_index.set(current_index + 1);
                    } else {
                        spawn(async move {
                            use_navigator().push(Route::Summary);
                        });
                    }
                }, "NEXT" }
            }

            div { class: "file-info-container",
                FileInfoLeft { file: DUPS()[current_index()].clone() }
                FileInfoRight { file: DUPS()[current_index()].clone() }
            }

            div { class: "media-pair",
                div { class: "media-box",
                    MediaDisplayLeft { media: DUPS()[current_index()].clone() }
                    div { id: "left-progress", class: "progress-video" }
                    button { class: "delete-button", onclick: move |_| {
                        let media = DUPS()[current_index()].clone();
                        match media {
                            DuplicateMedia::PhotoMatchGroup(a) => {
                                let image_path = a.images[0].path.clone();
                                DELETE_QUEUE.write().0.push(image_path);
                            }
                            DuplicateMedia::VideoMatchGroup(a) => {
                                let first_path = a.duplicates().next().unwrap();
                                DELETE_QUEUE.write().0.push(first_path.to_path_buf());
                            }
                        }
                    }, "DELETE" }
                }

                match &DUPS()[current_index()] {
                    DuplicateMedia::PhotoMatchGroup(_) => {
                        rsx! {}
                    }
                    DuplicateMedia::VideoMatchGroup(_) => {
                        rsx! {
                            VideoControls {}
                        }
                    }
                }
                div { class: "media-box",
                    MediaDisplayRight { media: DUPS()[current_index()].clone() }
                    div { id: "right-progress", class: "progress-video" }
                    button { class: "delete-button", onclick: move |_| {
                        let media = DUPS()[current_index()].clone();
                        match media {
                            DuplicateMedia::PhotoMatchGroup(a) => {
                                let image_path = a.images[1].path.clone();
                                DELETE_QUEUE.write().0.push(image_path);
                            }
                            DuplicateMedia::VideoMatchGroup(a) => {
                                let first_path = a.duplicates().last().unwrap();
                                DELETE_QUEUE.write().0.push(first_path.to_path_buf());
                            }
                        }
                    }, "DELETE" }
                }
            }

            button { class: "ignore-button", onclick: |_| {}, "IGNORE" }
        }
    }
}

#[component]
fn MediaDisplayLeft(media: DuplicateMedia) -> Element {
    rsx! {
        div { class: "media-container",
            match media {
                DuplicateMedia::PhotoMatchGroup(a) => {
                    let image_path: &str = a.images[1].path.to_str().unwrap_or("");
                    let encoded_path = encode(image_path);
                    rsx! {
                        img { class: "media", src: "{encoded_path}" }
                    }
                }
                DuplicateMedia::VideoMatchGroup(a) => {
                    let first_path = a.duplicates().next().unwrap();
                    let encoded_path = encode(first_path.to_str().unwrap());
                    println!("A sinistra -> {:?}", first_path);
                    rsx! {
                        video {
                            id: "video-left",
                            class: "media",
                            src: "{encoded_path}",
                            autoplay: true,
                            controls: false,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MediaDisplayRight(media: DuplicateMedia) -> Element {
    rsx! {
        div { class: "media-container",
            match media {
                DuplicateMedia::PhotoMatchGroup(a) => {
                    let image_path: &str = a.images[1].path.to_str().unwrap_or("");
                    let encoded_path = encode(image_path);
                    rsx! {
                        img { class: "media", src: "{encoded_path}" }
                    }
                }
                DuplicateMedia::VideoMatchGroup(a) => {
                    let last_path = a.duplicates().last().unwrap();
                    let encoded_path = encode(last_path.to_str().unwrap());
                    rsx! {
                        video {
                            id: "video-right",
                            class: "media",
                            src: "{encoded_path}",
                            autoplay: true,
                            controls: false,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VideoControls() -> Element {
    rsx! {
        div { class: "controls-column",
            // PLAY BUTTON
            button {
                class: "circle-button",
                onclick: |_| async move {
                    document::eval(r#"
                        const rightVideo = document.getElementById('video-right');
                        if (rightVideo) {
                            rightVideo.play();
                        }

                        const leftVideo = document.getElementById('video-left');
                        if (leftVideo) {
                            leftVideo.play();
                        }
                    "#).await.unwrap();
                },
                svg {
                    class: "icon",
                    view_box: "0 0 384 512",
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M73 39c-14.8-9.1-33.4-9.4-48.5-.9S0 62.6 0 80L0 432c0 17.4 9.4 33.4 24.5 41.9s33.7 8.1 48.5-.9L361 297c14.3-8.7 23-24.2 23-41s-8.7-32.2-23-41L73 39z" }
                }
            }
            // PAUSE BUTTON
            button { class: "circle-button", onclick: |_| async move {
                document::eval(r#"
                        const rightVideo = document.getElementById('video-right');
                        if (rightVideo) {
                            rightVideo.pause();
                        }

                        const leftVideo = document.getElementById('video-left');
                        if (leftVideo) {
                            leftVideo.pause();
                        }
                    "#).await.unwrap();
                },
                svg {
                    class: "icon",
                    view_box: "0 0 320 512",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "currentColor",
                    path { d: "M48 64C21.5 64 0 85.5 0 112L0 400c0 26.5 21.5 48 48 48l32 0c26.5 0 48-21.5 48-48l0-288c0-26.5-21.5-48-48-48L48 64zm192 0c-26.5 0-48 21.5-48 48l0 288c0 26.5 21.5 48 48 48l32 0c26.5 0 48-21.5 48-48l0-288c0-26.5-21.5-48-48-48l-32 0z" }
                }
            }
            // START OVER BUTTON
            button { class: "circle-button", onclick: |_| async move {
                document::eval(r#"
                    const rightVideo = document.getElementById('video-right');
                        if (rightVideo) {
                            rightVideo.pause();
                            rightVideo.currentTime = 0;
                            rightVideo.play();
                        }

                        const leftVideo = document.getElementById('video-left');
                        if (leftVideo) {
                            leftVideo.pause();
                            leftVideo.currentTime = 0;
                            leftVideo.play();
                        }
                "#).await.unwrap();
            },
                svg {
                    class: "icon",
                    view_box: "0 0 512 512",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "currentColor",
                    path { d: "M0 224c0 17.7 14.3 32 32 32s32-14.3 32-32c0-53 43-96 96-96l160 0 0 32c0 12.9 7.8 24.6 19.8 29.6s25.7 2.2 34.9-6.9l64-64c12.5-12.5 12.5-32.8 0-45.3l-64-64c-9.2-9.2-22.9-11.9-34.9-6.9S320 19.1 320 32l0 32L160 64C71.6 64 0 135.6 0 224zm512 64c0-17.7-14.3-32-32-32s-32 14.3-32 32c0 53-43 96-96 96l-160 0 0-32c0-12.9-7.8-24.6-19.8-29.6s-25.7-2.2-34.9 6.9l-64 64c-12.5 12.5-12.5 32.8 0 45.3l64 64c9.2 9.2 22.9 11.9 34.9 6.9s19.8-16.6 19.8-29.6l0-32 160 0c88.4 0 160-71.6 160-160z" }
                }
            }
            // SPEED BUTTON
            button { class: "circle-button", onclick: |_| async move {
                document::eval(r#"
                        const rightVideo = document.getElementById('video-right');
                        if (rightVideo) {
                            if (rightVideo.playbackRate === 1) {
                                rightVideo.playbackRate = 1.5;
                            } else {
                                rightVideo.playbackRate = 1;
                            }
                        }

                        const leftVideo = document.getElementById('video-left');
                        if (leftVideo) {
                            if (leftVideo.playbackRate === 1) {
                                leftVideo.playbackRate = 1.5;
                            } else {
                                leftVideo.playbackRate = 1;
                            }
                        }
                "#).await.unwrap();
            },
                svg {
                    class: "icon",
                    view_box: "0 0 512 512",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "currentColor",
                    path { d: "M0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256zm320 96c0-26.9-16.5-49.9-40-59.3L280 88c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 204.7c-23.5 9.5-40 32.5-40 59.3c0 35.3 28.7 64 64 64s64-28.7 64-64zM144 176a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm-16 80a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm288 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM400 144a32 32 0 1 0 -64 0 32 32 0 1 0 64 0z" }
                }
            }
            // AUDIO BUTTON
            button { class: "circle-button", onclick: |_| async move {
                    document::eval(r#"
                        const rightVideo = document.getElementById('video-right');
                        if (rightVideo) {
                            rightVideo.muted = !rightVideo.muted;
                        }

                        const leftVideo = document.getElementById('video-left');
                        if (leftVideo) {
                            leftVideo.muted = !leftVideo.muted;
                        }
                    "#).await.unwrap();
                },
                svg {
                    class: "icon",
                    view_box: "0 0 576 512",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "currentColor",
                    path { d: "M301.1 34.8C312.6 40 320 51.4 320 64l0 384c0 12.6-7.4 24-18.9 29.2s-25 3.1-34.4-5.3L131.8 352 64 352c-35.3 0-64-28.7-64-64l0-64c0-35.3 28.7-64 64-64l67.8 0L266.7 40.1c9.4-8.4 22.9-10.4 34.4-5.3zM425 167l55 55 55-55c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-55 55 55 55c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0l-55-55-55 55c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l55-55-55-55c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0z" }
                }
            }
        }
    }
}

#[component]
fn FileInfoLeft(file: DuplicateMedia) -> Element {
    rsx! {
        div {
            class: "file-info",
            div { class: "file-info-internal",
                match file {
                    DuplicateMedia::PhotoMatchGroup(a) => {
                        let image_path = a.images[0].path.clone();
                        let file_name = image_path.file_name().unwrap().to_str().unwrap();
                        let file_size = std::fs::metadata(&image_path).unwrap().file_size();
                        let real_size = format_size(file_size, DECIMAL);
                        rsx! {
                            span { class: "filename", "Name: {file_name}" }
                            span { "Size: {real_size}" }
                        }
                    }
                    DuplicateMedia::VideoMatchGroup(a) => {
                        let first_path = a.duplicates().next().unwrap();
                        let file_name = first_path.file_name().unwrap().to_str().unwrap();
                        let file_size = std::fs::metadata(first_path).unwrap().file_size();
                        let real_size = format_size(file_size, DECIMAL);
                        rsx! {
                            span { class: "filename", "Name: {file_name}" }
                            span { "Size: {real_size}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FileInfoRight(file: DuplicateMedia) -> Element {
    rsx! {
        div {
            class: "file-info",
            div { class: "file-info-internal",
                match file {
                    DuplicateMedia::PhotoMatchGroup(a) => {
                        let image_path = a.images[1].path.clone();
                        let file_name = image_path.file_name().unwrap().to_str().unwrap();
                        let file_size = std::fs::metadata(&image_path).unwrap().file_size();
                        let real_size = format_size(file_size, DECIMAL);
                        rsx! {
                            span { class: "filename", "Name: {file_name}" }
                            span { "Size: {real_size}" }
                        }
                    }
                    DuplicateMedia::VideoMatchGroup(a) => {
                        let first_path = a.duplicates().last().unwrap();
                        let file_name = first_path.file_name().unwrap().to_str().unwrap();
                        let file_size = std::fs::metadata(first_path).unwrap().file_size();
                        let real_size = format_size(file_size, DECIMAL);
                        rsx! {
                            span { class: "filename", "Name: {file_name}" }
                            span { "Size: {real_size}" }
                        }
                    }
                }
            }
        }
    }
}
