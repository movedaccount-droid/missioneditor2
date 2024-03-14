#![allow(non_snake_case)]

mod playmission;
mod components;
mod three;
mod utils;

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;

use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use crate::components::{ FilePicker, File };
use crate::playmission::{ MissionObject, Object, Value };

fn main() {
    // launch the web app
    launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App() -> Element {

    let mut import = use_signal(|| File::None);
    let mut mission = use_signal(|| None);
    let mut objects = use_signal(|| None);

    if matches!(*import.read(), File::Loaded{..}) {
        
        let File::Loaded { data, .. } = import.replace(File::None) else { unreachable!() };

        let cursor = Cursor::new(data);
        let (loaded_mission, loaded_objects) = MissionObject::deserialize(cursor).unwrap();
        *mission.write() = Some(loaded_mission);
        *objects.write() = Some(loaded_objects);

    }

    let text = mission.with(|m| {
        if let Some(m) = m {
            if let Value::String(s) = m.properties().get_value("Meta").unwrap() {
                s.clone()
            } else {
                "failed string?".to_owned()
            }
        } else {
            "failed load?".to_owned()
        }
    });

    rsx! {
        div { "{text}" },
        FilePicker { signal: import }
    }
}