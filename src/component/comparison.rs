use dioxus::prelude::*;

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
                        svg {
                            class: "icon",
                            view_box: "0 0 384 512",
                            fill: "currentColor",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M73 39c-14.8-9.1-33.4-9.4-48.5-.9S0 62.6 0 80L0 432c0 17.4 9.4 33.4 24.5 41.9s33.7 8.1 48.5-.9L361 297c14.3-8.7 23-24.2 23-41s-8.7-32.2-23-41L73 39z" }
                        }
                    }
                    button { class: "circle-button", onclick: |_| {},
                        svg {
                            class: "icon",
                            view_box: "0 0 320 512",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            path { d: "M48 64C21.5 64 0 85.5 0 112L0 400c0 26.5 21.5 48 48 48l32 0c26.5 0 48-21.5 48-48l0-288c0-26.5-21.5-48-48-48L48 64zm192 0c-26.5 0-48 21.5-48 48l0 288c0 26.5 21.5 48 48 48l32 0c26.5 0 48-21.5 48-48l0-288c0-26.5-21.5-48-48-48l-32 0z" }
                        }
                    }
                    button { class: "circle-button", onclick: |_| {},
                        svg {
                            class: "icon",
                            view_box: "0 0 512 512",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            path { d: "M0 224c0 17.7 14.3 32 32 32s32-14.3 32-32c0-53 43-96 96-96l160 0 0 32c0 12.9 7.8 24.6 19.8 29.6s25.7 2.2 34.9-6.9l64-64c12.5-12.5 12.5-32.8 0-45.3l-64-64c-9.2-9.2-22.9-11.9-34.9-6.9S320 19.1 320 32l0 32L160 64C71.6 64 0 135.6 0 224zm512 64c0-17.7-14.3-32-32-32s-32 14.3-32 32c0 53-43 96-96 96l-160 0 0-32c0-12.9-7.8-24.6-19.8-29.6s-25.7-2.2-34.9 6.9l-64 64c-12.5 12.5-12.5 32.8 0 45.3l64 64c9.2 9.2 22.9 11.9 34.9 6.9s19.8-16.6 19.8-29.6l0-32 160 0c88.4 0 160-71.6 160-160z" }
                        }
                    }
                    button { class: "circle-button", onclick: |_| {},
                        svg {
                            class: "icon",
                            view_box: "0 0 512 512",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            path { d: "M0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256zm320 96c0-26.9-16.5-49.9-40-59.3L280 88c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 204.7c-23.5 9.5-40 32.5-40 59.3c0 35.3 28.7 64 64 64s64-28.7 64-64zM144 176a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm-16 80a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm288 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM400 144a32 32 0 1 0 -64 0 32 32 0 1 0 64 0z" }
                        }
                    }
                    button { class: "circle-button", onclick: |_| {},
                        svg {
                            class: "icon",
                            view_box: "0 0 576 512",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            path { d: "M301.1 34.8C312.6 40 320 51.4 320 64l0 384c0 12.6-7.4 24-18.9 29.2s-25 3.1-34.4-5.3L131.8 352 64 352c-35.3 0-64-28.7-64-64l0-64c0-35.3 28.7-64 64-64l67.8 0L266.7 40.1c9.4-8.4 22.9-10.4 34.4-5.3zM425 167l55 55 55-55c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-55 55 55 55c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0l-55-55-55 55c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l55-55-55-55c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0z" }
                        }
                    }
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
        div { class: "media-container" }
    }
}
