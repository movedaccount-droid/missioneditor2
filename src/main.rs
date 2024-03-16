#![allow(non_snake_case)]

mod playmission;
mod components;
mod three;
mod utils;
mod tea;

use dioxus::html::select;
// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;
use gloo_console::log;
use uuid::Uuid;

use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use crate::components::{ File, FilePicker, RightPanel, Selector, Viewport };
use crate::playmission::{MissionObject, Object };

fn main() {
    launch(App);
}

fn App() -> Element {

    // loading file
    let mut import = use_signal(|| File::None);
    if matches!(*import.read(), File::Loaded{..}) {

        let File::Loaded { data, .. } = import.replace(File::None) else { unreachable!() };


        log!("written");

    }

    rsx! {
        div { "testing" },
        // Viewport {}
        // FilePicker { signal: import },
        // Selector { objects: objects, selected: selected, selector_callback: select_new }
        // RightPanel { selected: selected },
    }
}