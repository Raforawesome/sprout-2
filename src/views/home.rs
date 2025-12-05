use dioxus::prelude::*;

use crate::DragBar;

#[derive(Clone, PartialEq, Copy)]
pub enum SidebarRoute {
    InstalledMods,
    BrowseMods,
    Settings,
    Help,
    None,
}

#[component]
pub fn HomeScreen() -> Element {
    let selected = use_signal(|| SidebarRoute::None);

    rsx! {
        DragBar {}

        // main screen frame
        div {  class: "bg-emerald-950 flex flex-row w-screen h-screen",
            Sidebar { selected }
        }
    }
}

#[component]
pub fn Sidebar(selected: Signal<SidebarRoute>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center w-2/10 border border-emerald-900 rounded-md",
            div { // logo container
                class: "mt-9 gap-4 flex flex-row items-center",
                img { src: asset!("assets/sprout@1x-opt.webp"), class: "w-auto h-10" },
                p { class: "text-3xl text-stone-50 font-bold", "Sprout" }
            }

            SidebarItem {
                icon: "inventory_2",
                label: "Installed Mods",
                class: "mt-8",
                selected: selected() == SidebarRoute::InstalledMods,
                onclick: move |_| selected.set(SidebarRoute::InstalledMods),
            }

            SidebarItem {
                icon: "globe",
                label: "Browse Mods",
                selected: selected() == SidebarRoute::BrowseMods,
                onclick: move |_| selected.set(SidebarRoute::BrowseMods),
            }

            SidebarItem {
                icon: "settings",
                label: "Settings",
                selected: selected() == SidebarRoute::Settings,
                onclick: move |_| selected.set(SidebarRoute::Settings),
            }

            SidebarItem {
                icon: "help",
                label: "Help",
                selected: selected() == SidebarRoute::Help,
                onclick: move |_| selected.set(SidebarRoute::Help),
            }
        }
    }
}

#[component]
pub fn SidebarItem(
    onclick: EventHandler,
    #[props(into)] icon: String,
    #[props(into)] label: String,
    selected: bool,
    class: Option<String>,
) -> Element {
    let base_classes = "flex flex-row pl-6 gap-3 text-stone-200 font-medium py-3 w-full transition-all duration-100";
    let state_classes = if selected {
        "bg-lime-800"
    } else {
        "hover:bg-emerald-800 hover:scale-100"
    };
    let extra_classes = class.unwrap_or_default();

    rsx! {
        div {
            class: "{base_classes} {state_classes} {extra_classes}",
            onclick: move |_| onclick.call(()),

            span { class: "material-symbols-outlined", "{icon}" }
            p { "{label}" }
        }
    }
}
