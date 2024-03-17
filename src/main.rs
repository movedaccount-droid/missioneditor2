#![allow(non_snake_case)]

mod playmission;
mod components;
mod three;
mod utils;
mod tea;

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;
use gloo_console::log;
use gloo_file::{Blob, ObjectUrl};
use js_sys::Uint8Array;
use uuid::Uuid;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsCast;

use crate::components::{ File, FilePicker, Viewport };
use crate::playmission::Value;
use crate::tea::TeaHandler;
use crate::three::ImageLoader;

fn main() {
    launch(App);
}

fn App() -> Element {

    // loading file
    let mut import = use_signal(|| File::None);
    let tea_signal = use_signal(|| None);
    let mut tea = use_context_provider(|| tea_signal);
    let selected_signal = use_signal(|| Uuid::nil());
    let mut selected = use_context_provider(|| selected_signal);
    
    // seems to be the best way to do this... For Real ??...
    if matches!(*import.read(), File::Loaded{..}) {

        let File::Loaded { data, .. } = import.replace(File::None) else { unreachable!() };
        if let Ok(th) = TeaHandler::from_buffer(data) {
            *tea.write() = Some(th);
        }

    }

    let selected_file_key = use_signal(|| None);
    let save_closure = move |_| tea.write().iter_mut().next().unwrap().event(tea::Event::Save);

    rsx! {
        Viewport{},
        {tea.with_mut(|tea| {
            if let Some(tea) = tea {
                rsx! {
                    a {
                        onclick: save_closure,
                        "save"
                    }
                    p {
                        "{tea.display_status():?}"
                    },
                    for (uuid, name) in tea.display_objects() {
                        ObjectListing {uuid: uuid, name, selected_signal: selected}
                    }
                    if let Ok(properties) = tea.display_properties(*selected.read()) {
                        for (name, value) in properties {
                            if let Value::Bool(b) = value {
                                PropertyListingBool {name, value: *b, }
                            } else {
                                PropertyListingString {name, value: value.to_string()}
                            }
                        }
                    } else {
                        p {
                            "no properties"
                        }
                    }
                    {selected_file_key.with(|key_option| {
                        rsx! {
                            if let Some(key) = key_option {
                                FileBack { file_signal: selected_file_key }
                                if let Ok(buf) = tea.display_file(*selected.read(), key) {
                                    FileViewer{ buf: buf.to_owned() }
                                }
                            } else {
                                if let Ok(files) = tea.display_files(*selected.read()) {
                                    for file_key in files {
                                        FileListing {file_key, file_signal: selected_file_key}
                                    }
                                } else {
                                    p {
                                        "no files"
                                    }
                                }
                            }
                        }
                    })}
                }
            } else {
                rsx! {
                    FilePicker { signal: import }
                }
            }
        })}
    }

}

#[component]
fn ObjectListing(uuid: Uuid, name: String, selected_signal: Signal<Uuid>) -> Element {
    rsx! {
        a {
            onclick: move |_| *selected_signal.write() = uuid,
            "{name}"
        }
        br {}
    }
}

#[component]
fn PropertyListingBool(name: String, value: bool) -> Element {
    let mut tea = use_context::<Signal<Option<TeaHandler>>>();
    let selected = use_context::<Signal<Uuid>>();
    let cloned_name = name.clone();
    let closure = move |js_event: Event<FormData>| {
        tea.write().iter_mut().next().unwrap().event(
            tea::Event::UpdateProperty{
                uuid: *selected.read(),
                key: cloned_name.clone(),
                value: js_event.value()
            }
        )
    };
    rsx! {
        input {
            r#type: "checkbox",
            name: name.clone(),
            checked: value,
            onchange: closure,
        }
        label {
            r#for: name,
            "{name}"
        }
        br {}
    }
}

#[component]
fn PropertyListingString(name: String, value: String) -> Element {
    let mut tea = use_context::<Signal<Option<TeaHandler>>>();
    let selected = use_context::<Signal<Uuid>>();
    let cloned_name = name.clone();
    let closure = move |js_event: Event<FormData>| {
        log!("aclled");
        tea.write().iter_mut().next().unwrap().event(
            tea::Event::UpdateProperty{
                uuid: *selected.read(),
                key: cloned_name.clone(),
                value: js_event.value()
            }
        )
    };
    rsx! {
        input {
            r#type: "text",
            name: name.clone(),
            value: value,
            onchange: closure,
        }
        label {
            r#for: name,
            "{name}"
        }
        br {}
    }
}

#[component]
fn FileListing(file_key: String, file_signal: Signal<Option<String>>) -> Element {
    rsx! {
        a {
            onclick: move |_| *file_signal.write() = Some(file_key.clone()),
            "{file_key}"
        }
        br {}
    }
}

#[component]
fn FileBack(file_signal: Signal<Option<String>>) -> Element {
    rsx! {
        a {
            onclick: move |_| *file_signal.write() = None,
            "back"
        }
    }
}

#[component]
// hurting me and hurting me and hurting me
fn FileViewer(buf: Vec<u8>) -> Element {

    rsx! {
        div {
            id: "file-container",
            iframe {
                display: "none",
                width: 0,
                height: 0,
                onload: move |_| { file_viewer_init(&*buf); }
            }
        }
    }

}

fn file_viewer_init(buf: &[u8]) {
    log!("sdjhoiah");
    let window = web_sys::window().expect("no window found");
    let document = window.document().expect("no document found");
    let blob = Blob::new_with_options(&*buf, Some("image/x-targa"));
    let object_url = ObjectUrl::from(blob);

    let callback = |image| {
        log!("hit");
        let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        context.draw_image_with_html_image_element(&image, 100.0, 100.0).unwrap();
        log!("hit2");
        let data_url = canvas.to_data_url().unwrap();
        let img = document.create_element("img").unwrap();
        img.set_attribute("src", &*data_url).unwrap();
        let container = document.get_element_by_id("file-container").expect("no file container found");
        container.append_child(&img).expect("failed append");
        log!("hit3");
    };

    let error_callback = || {
        log!("errored");
    };
    
    let image_loader = ImageLoader::new();
    image_loader.load(&object_url, &callback, (), &error_callback);
}