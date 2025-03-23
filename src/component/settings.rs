use crate::{SETTINGS, SETTINGS_FILE};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "settings-container",
            div { class: "settings-card",
                h1 { class: "settings-title", "Settings" }

                div { class: "setting-item",
                    label { "Image similarity tolerance" }
                    input {
                        r#type: "range",
                        min: "5",
                        max: "20",
                        value: "{SETTINGS().image_tolerance()}",
                        oninput: move |e| {
                            SETTINGS.write().set_image_tolerance(e.data.value().parse().unwrap());
                        },
                    }
                }

                div { class: "settings-item",
                    label { "Video similarity tolerance" }
                    input {
                        r#type: "range",
                        min: "0.30",
                        max: "1.0",
                        step: "0.05",
                        value: "{SETTINGS().video_tolerance()}",
                        oninput: move |e| {
                            SETTINGS.write().set_video_tolerance(e.data.value().parse().unwrap());
                        },
                    }
                }

                div { class: "settings-item",
                    label { "Playback speed" }
                    select {
                        value: "{SETTINGS().playback_speed()}",
                        onchange: move |e| {
                            SETTINGS.write().set_playback_speed(e.data.value().parse().unwrap());
                        },
                        option { value: "0.5", "0.5x" }
                        option { value: "1", "1.0x" }
                        option { value: "1.5", "1.5x" }
                        option { value: "2.0", "2.0x" }
                    }
                }
                div { class: "settings-item",
                    label { "Autoplay video" }
                    input {
                        r#type: "checkbox",
                        checked: "{SETTINGS().autoplay_video()}",
                        onchange: move |e| {
                            SETTINGS.write().set_autoplay_video(e.data.value().parse().unwrap());
                        },
                    }
                }

                div { class: "settings-item",
                    label { "Theme" }
                    select {
                        value: "{SETTINGS().theme().to_str()}",
                        onchange: move |e| {
                            SETTINGS.write().set_theme(e.data.value().as_str());
                        },
                        option { value: "Light", "Light (Catppuccin Latte)" }
                        option { value: "Dark", "Dark (Catppuccin Frappe)" }
                    }
                }

                div { class: "settings-item",
                    label { "Accent" }
                    select {
                        value: "{SETTINGS().accent().to_str()}",
                        onchange: move |e| {
                            SETTINGS.write().set_accent(e.data.value().as_str());
                        },
                        option { value: "Blue", "Blue" }
                        option { value: "Green", "Green" }
                        option { value: "Red", "Red" }
                        option { value: "Yellow", "Yellow" }
                        option { value: "Pink", "Pink" }
                        option { value: "Mauve", "Mauve" }
                        option { value: "Peach", "Peach" }
                    }
                }

                div { class: "settings-item",
                    label { "Language" }
                    select {
                        value: "SETTINGS().language().to_str()",
                        onchange: move |_| {},
                        option { value: "English", "English" }
                        option { value: "Italiano", "Italiano" }
                    }
                }

                button {
                    class: "save-button",
                    onclick: move |_| {
                        SETTINGS().save_settings();
                    },
                    "SAVE"
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    image_tolerance: u32,
    video_duration: f64,
    video_tolerance: f64,
    mute_video: bool,
    playback_speed: f32,
    autoplay_video: bool,
    theme: Theme,
    accent: Accent,
    language: Language,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            image_tolerance: 10,
            video_duration: 2.0,
            video_tolerance: 0.35,
            mute_video: false,
            playback_speed: 1.0,
            autoplay_video: true,
            theme: Theme::Light,
            accent: Accent::Blue,
            language: Language::English,
        }
    }
}

#[allow(dead_code)]
impl AppSettings {
    pub fn image_tolerance(&self) -> u32 {
        self.image_tolerance
    }

    pub fn set_image_tolerance(&mut self, tol: u32) {
        self.image_tolerance = tol;
    }

    pub fn video_duration(&self) -> f64 {
        self.video_duration
    }

    pub fn set_video_duration(&mut self, duration: f64) {
        self.video_duration = duration;
    }

    pub fn video_tolerance(&self) -> f64 {
        self.video_tolerance
    }

    pub fn set_video_tolerance(&mut self, tol: f64) {
        self.video_tolerance = tol;
    }

    pub fn mute_video(&self) -> bool {
        self.mute_video
    }

    pub fn set_mute_video(&mut self, mute: bool) {
        self.mute_video = mute;
    }

    pub fn playback_speed(&self) -> f32 {
        self.playback_speed
    }

    pub fn set_playback_speed(&mut self, speed: f32) {
        self.playback_speed = speed;
    }

    pub fn autoplay_video(&self) -> bool {
        self.autoplay_video
    }

    pub fn set_autoplay_video(&mut self, autoplay: bool) {
        self.autoplay_video = autoplay;
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn set_theme(&mut self, t: &str) {
        match t {
            "Light" => {
                self.theme = Theme::Light;
            }
            "Dark" => {
                self.theme = Theme::Dark;
            }
            _ => {}
        }
    }

    pub fn accent(&self) -> &Accent {
        &self.accent
    }

    pub fn set_accent(&mut self, a: &str) {
        match a {
            "Blue" => self.accent = Accent::Blue,
            "Green" => self.accent = Accent::Green,
            "Red" => self.accent = Accent::Red,
            "Yellow" => self.accent = Accent::Yellow,
            "Pink" => self.accent = Accent::Pink,
            "Mauve" => self.accent = Accent::Mauve,
            "Peach" => self.accent = Accent::Peach,
            _ => {}
        }
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn save_settings(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(SETTINGS_FILE, json);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn to_str(&self) -> String {
        match self {
            Theme::Light => String::from("Light"),
            Theme::Dark => String::from("Dark"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    English,
    Italian,
}

#[allow(dead_code)]
impl Language {
    pub fn to_str(&self) -> String {
        match self {
            Language::English => String::from("English"),
            Language::Italian => String::from("Italian"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Accent {
    Blue,
    Green,
    Red,
    Yellow,
    Pink,
    Mauve,
    Peach,
}

impl Accent {
    pub fn to_str(&self) -> String {
        match self {
            Accent::Blue => String::from("Blue"),
            Accent::Green => String::from("Green"),
            Accent::Red => String::from("Red"),
            Accent::Yellow => String::from("Yellow"),
            Accent::Pink => String::from("Pink"),
            Accent::Mauve => String::from("Mauve"),
            Accent::Peach => String::from("Peach"),
        }
    }
}
