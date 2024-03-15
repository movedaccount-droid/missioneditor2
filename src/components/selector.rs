use dioxus::prelude::*;
use gloo_console::log;
use uuid::Uuid;

use crate::playmission::Object;


#[component]
pub fn Selector(objects: Signal<Option<std::collections::HashMap<uuid::Uuid, Object>>>, selected: Signal<Option<Object>>) -> Element {

    let uuid = if let Some(object) = &*selected.read() {
        Some(object.uuid().clone())
    } else {
        None
    };

    if let Some(objects) = &*objects.read() {
        rsx!(

            {objects.values().map(|object| {
                let name = object.name().unwrap_or("{unnamed object}".into());
                log!(object.uuid().to_string());
                log!(uuid.unwrap().to_string());
                let color = if Some(object.uuid()) == uuid.as_ref() {
                    "gray"
                } else {
                    "yellow"
                };
                rsx!{ a {
                    color: color,
                    "{name}"
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