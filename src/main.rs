#![allow(non_snake_case)]

mod playmission;
mod components;
mod three;
mod utils;

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;

use std::io::Cursor;

use crate::components::{ File, FilePicker, RightPanel, Viewport };
use crate::playmission::MissionObject;

fn main() {
    // launch the web app
    launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App() -> Element {

    let mut import = use_signal(|| File::None);
    let mut mission = use_signal(|| None);
    let mut objects = use_context_provider(|| Signal::new(None));

    if matches!(*import.read(), File::Loaded{..}) {

        let File::Loaded { data, .. } = import.replace(File::None) else { unreachable!() };

        let cursor = Cursor::new(data);
        let (loaded_mission, loaded_objects) = MissionObject::deserialize(cursor).unwrap();
        *mission.write() = Some(loaded_mission);
        *objects.write() = Some(loaded_objects);

    }

    rsx! {
        div { "testing" },
        Viewport {}
        FilePicker { signal: import },
        RightPanel {},
    }
}