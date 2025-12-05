use std::path::PathBuf;

use dioxus::prelude::*;
use rfd::FileDialog;

use crate::{DragBar, settings::get_settings};

#[component]
pub fn ImportScreen() -> Element {
    // 3 possible states:
    // - None means do not show error text
    // - Some(false) means show success text
    // - Some(true) means show error text
    let mut has_error: Signal<Option<bool>> = use_signal(|| None);
    let mut picker_path = use_signal(|| mods_path().to_string_lossy().to_string());

    if default_game_path().exists() {
        has_error.set(Some(false));
    }

    rsx! {
        DragBar {}

        div { // view root container
            class: "bg-emerald-950 w-screen h-screen flex items-center justify-center",
            div { // centered inner container
                class: "flex flex-col items-center justify-center",
                p { class: "text-3xl font-bold text-stone-50", "Where is Stardew Valley installed?" }
                p { class: "text-md text-stone-400", "If you have a non-standard install directory, you may need to configure this." },

                div { // file selector container
                    class: "flex flex-col align-left w-full",

                    p { class: "mt-10 text-md text-stone-50", "Game Path" }
                    input { class: "mt-1 h-10 w-full border-1 border-stone-400 bg-green-900 rounded-xl p-2 text-stone-100",
                        oninput: move |ev| picker_path.set(ev.value()),
                        value: picker_path()
                    }

                    p { // error text below the input box
                        class: {if let Some(has_error) = has_error() {
                            if has_error {
                                "text-red-500 mt-1"
                            } else {
                                "text-green-300 mt-1"
                            }
                        } else {""}},
                        hidden: has_error().is_none(),
                        {
                        match has_error() {
                            Some(has_error) => if has_error { "Path not found" } else { "Path found!" },
                            None => "",
                        }
                    } }

                    // input box buttons
                    div { // container for buttons
                        class: "mt-4 flex flex-row gap-4",
                        button { class: "bg-green-700 rounded-xl h-12 w-4/10 text-stone-50 font-semibold hover:bg-green-600 hover:shadow-lg hover:scale-105 transition-all duration-200",
                            div { class: "flex flex-row gap-2 items-center justify-center",
                                onclick: move |_| {
                                    let picked: Option<PathBuf> = FileDialog::new()
                                        .set_title("Find your Stardew Valley installation")
                                        .pick_folder();

                                    if let Some(path) = picked {
                                        picker_path.set(path.to_string_lossy().to_string());
                                    }
                                },
                                span { class: "material-symbols-outlined", "folder" },
                                "Browse"
                            }
                        }

                        button { class: "bg-lime-600 rounded-xl h-12 w-6/10 text-stone-50 font-semibold hover:bg-lime-500 hover:shadow-lg hover:scale-105 transition-all duration-200",
                            onclick: move |_| {
                                if std::fs::exists(picker_path.read().as_str()).is_ok_and(|b| b) {
                                    has_error.set(Some(false));
                                    get_settings().set_game_path(Some(PathBuf::from(picker_path())));
                                    navigator().push("/home");
                                } else {
                                    has_error.set(Some(true));
                                }
                            },
                            "Confirm"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn default_game_path() -> PathBuf {
    PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\common\Stardew Valley")
}

#[cfg(target_os = "macos")]
fn default_game_path() -> PathBuf {
    dirs::config_dir()
        .expect("Valid Application Support directory")
        .join("Steam/SteamApps/common/Stardew Valley/Contents/MacOS")
}

fn mods_path() -> PathBuf {
    default_game_path().join("Mods")
}
