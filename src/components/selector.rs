use std::{cell::RefCell, iter};

use dioxus::prelude::*;
use gloo_console::log;
use uuid::Uuid;

use crate::playmission::Object;


#[component]
pub fn Selector(
    objects: Signal<Option<std::collections::HashMap<uuid::Uuid, Object>>>,
    selected: Signal<Option<Object>>,
    selector_callback: Signal<RefCell<Box<dyn FnMut(&Uuid)>>>
) -> Element {

    rsx!(
        p {
            "loadeig"
        }
    )

}