use std::path::PathBuf;

use component::settings::{Accent, AppSettings, Theme};
use dioxus::prelude::*;

mod component;
mod model;

use component::comparison::Comparison;
use component::home::Home;
use component::loading::Loading;
use component::nodups::NoDups;
use component::settings::Settings;
use component::summary::Summary;
use model::media::DuplicateMedia;

const SETTINGS_FILE: &str = "settings.json";

static MAIN_CSS: Asset = asset!("/assets/main.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");

static SETTINGS: GlobalSignal<AppSettings> = Global::new(load_settings);
static DUPS: GlobalSignal<Vec<DuplicateMedia>> = Global::new(Vec::new);
static DELETE_QUEUE: GlobalSignal<DeleteQueue> = Global::new(|| DeleteQueue(Vec::new()));
static IGNORE_QUEUE: GlobalSignal<IgnoreDuplicate> = Global::new(|| IgnoreDuplicate(Vec::new()));

fn main() {
    #[cfg(feature = "desktop")]
    fn launch_app() {
        use dioxus::desktop::{tao, LogicalSize};
        let window = tao::window::WindowBuilder::new()
            .with_resizable(true)
            .with_title("Analyzer")
            .with_min_inner_size(LogicalSize::new(650, 700));
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::new()
                    .with_window(window)
                    .with_menu(None),
            )
            .launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    fn launch_app() {
        dioxus::launch(App);
    }
    launch_app();
}

#[component]
fn App() -> Element {
    use_effect(move || {
        let theme_js = match SETTINGS().theme() {
            Theme::Light => r#"document.documentElement.setAttribute('data-theme', 'latte');"#,
            Theme::Dark => r#"document.documentElement.setAttribute('data-theme', 'frappe');"#,
        };

        let accent_js: &str = match SETTINGS().accent() {
            Accent::Blue => r#"document.documentElement.setAttribute('data-accent','blue');"#,
            Accent::Green => r#"document.documentElement.setAttribute('data-accent','green');"#,
            Accent::Red => r#"document.documentElement.setAttribute('data-accent','red');"#,
            Accent::Yellow => r#"document.documentElement.setAttribute('data-accent','yellow');"#,
            Accent::Pink => r#"document.documentElement.setAttribute('data-accent','pink');"#,
            Accent::Peach => r#"document.documentElement.setAttribute('data-accent','peach');"#,
            Accent::Mauve => r#"document.documentElement.setAttribute('data-accent','mauve');"#,
        };

        spawn(async move {
            document::eval(theme_js).await.unwrap();
            document::eval(accent_js).await.unwrap();
        });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home,

    #[route("/loading/:folder_path")]
    Loading { folder_path: String },

    #[route("/comparison")]
    Comparison,

    #[route("/nodups")]
    NoDups,

    #[route("/summary")]
    Summary,

    #[route("/settings")]
    Settings,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteQueue(Vec<PathBuf>);
#[derive(Debug, Clone, PartialEq)]
pub struct IgnoreDuplicate(Vec<DuplicateMedia>);

fn load_settings() -> AppSettings {
    if let Ok(contents) = std::fs::read_to_string(SETTINGS_FILE) {
        if let Ok(settings) = serde_json::from_str::<AppSettings>(&contents) {
            return settings;
        }
    }

    AppSettings::default()
}
