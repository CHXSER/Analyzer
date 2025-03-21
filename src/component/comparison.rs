use dioxus::prelude::*;

//const PIC: Asset = asset!("/assets/pic.jpg");
const VIDEO: Asset = asset!("/assets/video.mp4");
const PLAY_ICO: Asset = asset!("/assets/icons/play.svg");
const PAUSE_ICO: Asset = asset!("/assets/icons/pause.svg");
const REPEAT_ICO: Asset = asset!("/assets/icons/repeat.svg");

#[component]
pub fn Comparison() -> Element {
    rsx! {
        div { id: "duplicate-container",
            div { class: "nav-bar",
                button { class: "nav-button prev", onclick: |_| println!("Ciao!"), "PREVIOUS" }
                div { class: "counter", "1 of 10 duplicates" }
                button { class: "nav-button next", onclick: |_| println!("Ciao!"), "NEXT" }
            }

            div { class: "file-info-container",
                div { class: "file-info", "File info 1" }
                div { class: "file-info", "File info 2" }
            }

            div { class: "media-pair",
                div { class: "media-box",
                    MediaDisplay { id: "video-left" }
                    div { id: "left-progress", class: "progress-video",
                        "0:00"
                        input {
                            class: "progress-bar-video",
                            r#type: "range",
                            min: "0",
                            max: "100",
                            value: "0",
                        }
                        "0:00"
                    }
                    button { class: "delete-button", onclick: |_| {}, "DELETE" }
                }

                div { class: "controls-column",
                    button { class: "circle-button", onclick: |_| {},
                        svg { class: "icon", xmlns: PLAY_ICO }
                    }
                    button { class: "circle-button", onclick: |_| {},
                        svg { class: "icon", xmlns: PAUSE_ICO }
                    }
                    button { class: "circle-button", onclick: |_| {},
                        svg { class: "icon", xmlns: REPEAT_ICO }
                    }
                    button { class: "circle-button", onclick: |_| {} }
                    button { class: "circle-button", onclick: |_| {} }
                }

                div { class: "media-box",
                    MediaDisplay { id: "video-right" }
                    div { id: "right-progress", class: "progress-video",
                        "1:24"
                        input {
                            class: "progress-bar-video",
                            r#type: "range",
                            min: "0",
                            max: "100",
                            value: "0",
                        }
                        "4:44"
                    }
                    button { class: "delete-button", onclick: |_| {}, "DELETE" }
                }
            }

            button { class: "ignore-button", onclick: |_| {}, "IGNORE" }
        }
    }
}

#[component]
fn MediaDisplay(id: &'static str) -> Element {
    rsx! {
        div { class: "media-container",
            video {
                class: "media",
                controls: false,
                autoplay: false,
                id: "{id}",
                src: VIDEO,
            }
        }
    }
}
