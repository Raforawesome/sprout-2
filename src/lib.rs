pub mod settings;
pub mod views;

use dioxus::desktop::window;
use dioxus::prelude::*;

#[component]
/// a thin invisible dragbar that exists at the top of the screen
/// outside of document flow, making the window drag when clicked.
pub fn DragBar() -> Element {
    rsx! {
        div {
            onmousedown: |_| window().drag(),
            class: "fixed opacity-0 w-full h-10 cursor-pointer"
        }
    }
}
