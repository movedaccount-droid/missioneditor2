#![allow(non_snake_case)]

mod playmission;
mod components;
mod three;
mod utils;
mod tea;

use std::io::Cursor;

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;
use gloo_console::log;
use image::ImageFormat;
use uuid::Uuid;
use image::io::Reader as ImageReader;
use base64::prelude::*;

use crate::components::{ File, FilePicker, Viewport };
use crate::playmission::Value;
use crate::tea::TeaHandler;
use crate::three::Scene;

const _TAILWIND_URL: &str = manganis::mg!(file("input.css"));

fn main() {
    launch(App);
}

fn App() -> Element {

    // storing three.js canvas for rendering
    let mut scene: Signal<Option<Scene>> = use_signal(|| None);

    // loading file
    let mut import = use_signal(|| File::None);
    let tea_signal = use_signal(|| None);
    let mut tea = use_context_provider(|| tea_signal);
    let selected_signal = use_signal(|| Uuid::nil());
    let selected = use_context_provider(|| selected_signal);
    
    // seems to be the best way to do this... For Real ??...
    if matches!(*import.read(), File::Loaded{..}) {

        let File::Loaded { data, .. } = import.replace(File::None) else { unreachable!() };
        if let Ok(mut th) = TeaHandler::from_buffer(data) {
            th.render((*scene.write()).iter_mut().next().unwrap());
            *tea.write() = Some(th);
        }

    }

    // various signals and setup for main page rendering
    let selected_file_key = use_signal(|| None);
    let save_closure = move |_| tea.write().iter_mut().next().unwrap().event(tea::Event::Save);
    let undo_closure = move |_| tea.write().iter_mut().next().unwrap().event(tea::Event::Undo);
    let redo_closure = move |_| tea.write().iter_mut().next().unwrap().event(tea::Event::Redo);

    // resource file importer
    let mut file_import = use_signal(|| File::None);
    if matches!(*file_import.read(), File::Loaded{..}) {

        let File::Loaded { data, .. } = file_import.replace(File::None) else { unreachable!() };
        tea.write().iter_mut().next().unwrap().event(tea::Event::UpdateFile { uuid: *selected.read(), key: (*selected_file_key.read()).clone().unwrap(), buffer: data })

    }

    rsx! {
        Viewport{ scene_signal: scene, selected_signal },
        {tea.with_mut(|tea| {
            if let Some(tea) = tea {
                rsx! {
                    a {
                        onclick: save_closure,
                        "save"
                    }
                    a {
                        onclick: undo_closure,
                        "undo"
                    }
                    a {
                        onclick: redo_closure,
                        "redo"
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
                                PropertyListingBool {name, value: *b, datafile: false}
                            } else {
                                PropertyListingString {name, value: value.to_string(), datafile: false}
                            }
                        }
                    } else {
                        p {
                            "no properties"
                        }
                    }
                    if let Ok(properties) = tea.display_datafile(*selected.read()) {
                        for (name, value) in properties {
                            if let Value::Bool(b) = value {
                                PropertyListingBool {name, value: *b, datafile: true}
                            } else {
                                PropertyListingString {name, value: value.to_string(), datafile: true}
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
                                    FilePicker{ signal: file_import }
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
fn PropertyListingBool(name: String, value: bool, datafile: bool) -> Element {
    log!(name.clone());
    let mut tea = use_context::<Signal<Option<TeaHandler>>>();
    let selected = use_context::<Signal<Uuid>>();
    let on_change = if datafile {
        datafile_update_closure(name.clone(), tea, selected)
    } else {
        property_update_closure(name.clone(), tea, selected)
    };
    rsx! {
        input {
            r#type: "checkbox",
            name: name.clone(),
            checked: value,
            onchange: on_change,
        }
        label {
            r#for: name,
            "{name}"
        }
        br {}
    }
}

#[component]
fn PropertyListingString(name: String, value: String, datafile: bool) -> Element {
    log!(name.clone());
    let mut tea = use_context::<Signal<Option<TeaHandler>>>();
    let selected = use_context::<Signal<Uuid>>();
    let on_change = if datafile {
        datafile_update_closure(name.clone(), tea, selected)
    } else {
        property_update_closure(name.clone(), tea, selected)
    };
    rsx! {
        input {
            r#type: "text",
            name: name.clone(),
            value: value,
            onchange: on_change,
        }
        label {
            r#for: name,
            "{name}"
        }
        br {}
    }
}

fn property_update_closure(name: String, mut tea: Signal<Option<TeaHandler>>, selected: Signal<Uuid>) -> Box<dyn FnMut(Event<FormData>)> {

    let cls = move |js_event: Event<FormData>| {
        tea.write().iter_mut().next().unwrap().event(
            tea::Event::UpdateProperty{
                uuid: *selected.read(),
                key: name.clone(),
                value: js_event.value()
            }
        )
    };
    Box::new(cls)

}

fn datafile_update_closure(name: String, mut tea: Signal<Option<TeaHandler>>, selected: Signal<Uuid>) -> Box<dyn FnMut(Event<FormData>)> {

    let cls = move |js_event: Event<FormData>| {
        tea.write().iter_mut().next().unwrap().event(
            tea::Event::UpdateDatafile{
                uuid: *selected.read(),
                key: name.clone(),
                value: js_event.value()
            }
        )
    };
    Box::new(cls)

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
fn FileViewer(buf: Vec<u8>) -> Element {

    let mut reader = ImageReader::new(Cursor::new(buf));
    reader.set_format(ImageFormat::Tga);
    let image = reader.decode().unwrap();

    let mut buf: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png).unwrap();

    let data_uri = format!("data:image/png;base64,{}", BASE64_STANDARD.encode(buf));

    rsx! {
        img {
            src: data_uri,
            width: 500,
            height: 500
        }
    }

}