#![allow(non_snake_case)]

mod playmission;
mod components;
mod three;
mod utils;

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;

use crate::components::{ FilePicker, File };

fn main() {
    // launch the web app
    launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App() -> Element {

    let import = use_signal(|| File::None);
    let text = use_memo(move || {
        match &*import.read() {
            File::Loaded{name, ..} => name.clone(),
            _ => "nothiung loadinged".to_owned(),
        }
    });

    rsx! {
        div { "{text}" },
        FilePicker { signal: import }
    }
}