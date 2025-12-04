use dioxus::prelude::*;
use dioxus_desktop::{
    Config, LogicalSize, WindowBuilder, tao::platform::macos::WindowBuilderExtMacOS,
};

use sprout_2::settings::{SETTINGS, get_settings};
use sprout_2::views::ImportScreen;

#[derive(Routable, Clone)]
enum Routes {
    #[route("/")]
    Loader,
    #[route("/import")]
    ImportScreen,
}

#[component]
fn Loader() -> Element {
    let navigator = navigator();
    let settings = get_settings();

    if let Some(game_path) = settings.game_path() {
        debug!("Found game path {game_path}");
        navigator.push("/home");
    } else {
        navigator.push("/import");
    }

    rsx! { "Loading app..." }
}

#[component]
fn App() -> Element {
    rsx! {
        Stylesheet { href: asset!("assets/main.css") }
        Stylesheet { href: asset!("assets/tailwind.css") }

        Router::<Routes> {}
    }
}

fn main() {
    // initialize tracing
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG)
        .expect("Failed to initialize logger!");

    // ensure settings path can be loaded (operation will panic with error message if unable)
    std::sync::LazyLock::force(&SETTINGS);

    LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_disable_context_menu(true).with_window(
                WindowBuilder::new()
                    .with_title("Sprout")
                    .with_fullsize_content_view(true)
                    .with_title_hidden(true)
                    .with_titlebar_transparent(true)
                    .with_resizable(true)
                    .with_inner_size(LogicalSize::new(1000, 685)),
            ),
        )
        .launch(App);
}
