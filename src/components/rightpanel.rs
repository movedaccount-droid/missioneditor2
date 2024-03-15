use std::{cell::RefCell, collections::HashMap, rc::Rc};

use dioxus::prelude::*;
use uuid::Uuid;

use crate::playmission::Object;


#[component]
pub fn RightPanel() -> Element {

    let context = consume_context::<Signal<HashMap<Uuid, Box<dyn Object>>>>();

    rsx!(
        p {
            "loading"
        }
    )

}