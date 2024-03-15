use std::collections::HashMap;

use dioxus::prelude::*;
use gloo_console::log;
use uuid::Uuid;

use crate::playmission::Object;


#[component]
pub fn RightPanel(selected: Signal<Option<Object>>) -> Element {

    log!("reload");

    if let Some(object) = &*selected.read() {
        rsx!(

            // display properties
            {object.properties().iter().map(|(key, property)| {
                let property_string = property.value().to_string();
                rsx!{ p {
                    "{key}: {property_string}"
                }}
            })}

            // display datafile
            {object.datafile().iter().map(|(key, property)| {
                let property_string = property.value().to_string();
                rsx!{ p {
                    "{key}: {property_string}"
                }}
            })}

            // display files
            {object.files().iter().map(|(key, file)| {
                let property_string = &file[0..100];
                rsx!{ p {
                    "{key}: {property_string:?}"
                }}
            })}

        )
    } else {
        rsx!(
            p {
                "loadeig"
            }
        )
    }

}