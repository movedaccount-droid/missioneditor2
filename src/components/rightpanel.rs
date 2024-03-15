use std::collections::HashMap;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::playmission::Object;


#[component]
pub fn RightPanel() -> Element {

    let context = consume_context::<Signal<Option<HashMap<Uuid, Object>>>>();
    let string = if let Some(ref m) = *context.read() {
        m.values().next().unwrap().uuid().to_string()
    } else {
        "loading".into()
    };

    rsx!(
        p {
            "{string}"
        }
    )

}