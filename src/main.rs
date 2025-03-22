use dioxus::prelude::*;

mod component;
mod model;

use component::comparison::Comparison;
use component::home::Home;
use component::loading::Loading;
use model::media::DuplicateMedia;

static LIGHT_MAIN_CSS: Asset = asset!("/assets/light_main.css");
static DARK_MAIN_CSS: Asset = asset!("/assets/dark_main.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");

static THEME: GlobalSignal<DarkTheme> = Global::new(|| DarkTheme(true));
static DUPS: GlobalSignal<Vec<DuplicateMedia>> = Global::new(Vec::new);

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
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        if THEME.read().0 {
            document::Stylesheet { href: DARK_MAIN_CSS }
        } else {
            document::Stylesheet { href: LIGHT_MAIN_CSS }
        }
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
}

struct DarkTheme(bool);
