use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
// TODO: Tolleranza immagini (int da 5 a 20?)
// TODO: Durata hash viedo (in secondi f64)
// TODO: Tolleranza video (float da 0.35 a 1.0?)
// TODO: Impostazioni audio video (muto o lascia audio)
// TODO: Impostazioni playback video (velocità di partenza 1.5x,2.0x,0.5x)
// TODO: Impostazioni autoplay video (se il video parte da solo o no)
// TODO: Impostazioni tema (Tema chiaro, scuro) e colore accento
// TODO: Impostazioni lingua (Inglese, Italiano)
#[component]
pub fn Settings() -> Element {
    rsx! {
        div { id: "container",
            h1 { "Settings" }
            div { id: "settings-group",
                label { "Image similarity tolerance" }
                input {
                    r#type: "range",
                    min: "5",
                    max: "20",
                    value: "",
                    oninput: move |_| {},
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

    pub fn video_duration(&self) -> f64 {
        self.video_duration
    }

    pub fn video_tolerance(&self) -> f64 {
        self.video_tolerance
    }

    pub fn mute_video(&self) -> bool {
        self.mute_video
    }

    pub fn playback_speed(&self) -> f32 {
        self.playback_speed
    }

    pub fn autoplay_video(&self) -> bool {
        self.autoplay_video
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn accent(&self) -> &Accent {
        &self.accent
    }

    pub fn language(&self) -> &Language {
        &self.language
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    English,
    Italian,
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
