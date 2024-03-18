use std::{collections::{HashMap, VecDeque}, io::Cursor};

use dioxus::{events::{keyboard_types::KeyboardEvent, Key, ModifiersInteraction}, html::KeyboardData};
use dioxus::events::Code;
use gloo_console::log;
use gloo_file::{Blob, ObjectUrl};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement};

use crate::{playmission::{
    error::PlaymissionError, MissionObject, Object, Value
}, three::Scene};

// manages The Elm Architecture for interfacing with the inner project
pub struct TeaHandler {
    missionobject: MissionObject,
    objects: HashMap<Uuid, Object>,
    status: Option<String>,
    undo_buffer: VecDeque<InverseEvent>,
    redo_buffer: VecDeque<Event>,
}

impl TeaHandler {

    // import a mission from a file
    pub fn from_buffer(data: Vec<u8>) -> std::result::Result<Self, TeaError> {
        let cursor = Cursor::new(data);
        let (missionobject, objects) = MissionObject::deserialize(cursor)?;
        Ok(Self {
            missionobject,
            objects,
            status: None,
            undo_buffer: VecDeque::new(),
            redo_buffer: VecDeque::new(),
        })
    }

    // entry point to start event chain
    pub fn event(&mut self, event: Event) {
        self.reset_state();
        let result = self.run_event(event);
        match result {
            Ok(Some(inverse_event)) => { self.push_new_undo(inverse_event) },
            Err(e) => { self.status = Some(e.to_string()) },
            _ => {},
        }
    }

    // matches and handles events
    fn run_event(&mut self, event: Event) -> UpdateResult {

        match event {
            Event::Save => self.save(),
            Event::Keypress{e} => self.keypress(e),
            Event::UpdateProperty{uuid, key, value} => self.update_property(uuid, key, value),
            Event::UpdateDatafile{uuid, key, value} => self.update_datafile(uuid, key, value),
            Event::UpdateFile{uuid, key, buffer} => self.update_file(uuid, key, buffer),
            Event::Undo => self.undo(),
            Event::Redo => self.redo(),
        }

    }

    // resets anything that should not persist between states
    fn reset_state(&mut self) {
        self.status = None
    }

    // export current mission to serialized Vec buffer
    fn save(&mut self) -> UpdateResult {
        // clone is kind of very gross
        let buf = self.missionobject.clone().serialize(self.objects.clone())?;
        let blob = Blob::new_with_options(&*buf, Some("application/zip"));
        let object_url = ObjectUrl::from(blob);
        let window = web_sys::window().expect("missing window");
        let document = window.document().expect("missing document");
        let link = document.create_element("a").map_err(|_| TeaError::FailedLinkCreation)?;
        link.set_attribute("download", "export.playmission").unwrap();
        link.set_attribute("href", &object_url).unwrap();
        let link_as_html: HtmlElement = link.dyn_into().unwrap();
        link_as_html.click();
        link_as_html.remove();
        Ok(None)
    }

    // handles keypresses, i.e. ctrl+z
    fn keypress(&mut self, e: web_sys::KeyboardEvent) -> UpdateResult {

        if e.code() == "KeyZ" {
            let event = if e.get_modifier_state("Shift"){
                Event::Redo
            } else {
                Event::Undo
            };
            self.run_event(event)
        } else {
            Ok(None)
        }

    }

    // updates property on an object by uuid
    fn update_property(&mut self, uuid: Uuid, key: String, value: impl Into<String>) -> UpdateResult {

        let object = self.get_object_mut(uuid)?;

        let old = object.set_property(key.clone(), value)?.unwrap();
        let inverse_event = Event::UpdateProperty { uuid, key: key.into(), value: old.to_string() };
        Ok(Some(InverseEvent(inverse_event)))

    }

    // updates datafile property on an object by uuid
    fn update_datafile(&mut self, uuid: Uuid, key: String, value: impl Into<String>) -> UpdateResult {

        let object = self.get_object_mut(uuid)?;

        let old = object.set_datafile(key.clone(), value)?.unwrap();
        let inverse_event = Event::UpdateDatafile { uuid, key: key.into(), value: old.to_string() };
        Ok(Some(InverseEvent(inverse_event)))

    }

    // updates file on an object by uuid
    fn update_file(&mut self, uuid: Uuid, key: impl AsRef<str> + Into<String>, buffer: Vec<u8>) -> UpdateResult {

        let object = self.get_object_mut(uuid)?;

        let old = object.set_file(key.as_ref(), buffer)?.unwrap();
        let inverse_event = Event::UpdateFile { uuid, key: key.into(), buffer: old };
        Ok(Some(InverseEvent(inverse_event)))

    }

    // undoes an event, if available
    fn undo(&mut self) -> UpdateResult {
        let event = self.undo_buffer.pop_front().ok_or(TeaError::NoUndo)?;
        let inverse_event = self.run_event(event.unwrap())?;
        if let Some(inverse) = inverse_event {
            self.push_redo(inverse.unwrap());
        }
        Ok(None)
    }

    // redoes an event, if available
    fn redo(&mut self) -> UpdateResult {
        let event = self.redo_buffer.pop_front().ok_or(TeaError::NoUndo)?;
        let inverse_event = self.run_event(event)?;
        if let Some(inverse) = inverse_event {
            self.push_undo(inverse);
        }
        Ok(None)
    }

    // pushes an inverseevent to the undo buffer
    fn push_undo(&mut self, event: InverseEvent) {
        self.undo_buffer.push_front(event);
        self.undo_buffer.truncate(200);
    }

    // pushes an inverseevent to the undo buffer,
    // clearing the redo buffer in the process
    fn push_new_undo(&mut self, event: InverseEvent) {
        self.push_undo(event);
        self.redo_buffer.clear();
    }

    // pushes event to redo buffer
    fn push_redo(&mut self, event: Event) {
        self.redo_buffer.push_front(event);
        self.redo_buffer.truncate(200);
    }

    // gets object by uuid or error
    fn get_object(&self, uuid: Uuid) -> ViewResult<&Object> {
        self.objects.get(&uuid).ok_or(TeaError::NoUuid(uuid))
    }

    // gets object mutably by uuid or error
    fn get_object_mut(&mut self, uuid: Uuid) -> ViewResult<&mut Object> {
        self.objects.get_mut(&uuid).ok_or(TeaError::NoUuid(uuid))
    }

    // returns vec of object names and uuids
    pub fn display_objects(&self) -> Vec<(Uuid, String)> {
        self.objects.iter()
        .map(|(k, v)| (k.clone(), v.name().unwrap_or("{unnamed object}".into())))
        .collect()
    }

    // returns (k, v) of property names and values
    pub fn display_properties(&self, uuid: Uuid) -> ViewResult<Vec<(String, &Value)>> {
        Ok(
            self.get_object(uuid)?
                .properties()
                .iter()
                .map(|(k, v)| (k.clone(), v.value()))
                .collect()
        )
    }

    // returns (k, v) of datafile names and values
    pub fn display_datafile(&self, uuid: Uuid) -> ViewResult<Vec<(&str, &Value)>> {
        Ok(
            self.get_object(uuid)?
                .datafile()
                .iter()
                .map(|(k, v)| (&**k, v.value()))
                .collect()
        )
    }

    // returns names of files on object by uuid
    pub fn display_files(&self, uuid: Uuid) -> ViewResult<Vec<String>> {
        Ok(
            self.get_object(uuid)?
                .files()
                .keys()
                .map(|k| k.clone())
                .collect()
        )
    }

    // returns a single datafile buffer by uuid
    pub fn display_file(&self, uuid: Uuid, key: impl AsRef<str>) -> ViewResult<&[u8]> {
        self.get_object(uuid)?
            .files()
            .get(key.as_ref())
            .map(|buf| &**buf)
            .ok_or(TeaError::NoFile)
    }

    // return status string
    pub fn display_status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    // renders all objects to three.js scene
    pub fn render(&mut self, scene: &mut Scene) {
        self.objects.values_mut().for_each(|object| { object.render(scene); })
    }

}

pub enum Event {
    Save,
    Keypress{e: web_sys::KeyboardEvent},
    UpdateProperty{uuid: Uuid, key: String, value: String},
    UpdateDatafile{uuid: Uuid, key: String, value: String},
    UpdateFile{uuid: Uuid, key: String, buffer: Vec<u8>},
    Undo,
    Redo,
}

struct InverseEvent(Event);

impl InverseEvent {
    fn unwrap(self) -> Event {
        self.0
    }
}

use thiserror::Error;

pub type UpdateResult = Result<Option<InverseEvent>, TeaError>;
pub type ViewResult<T> = Result<T, TeaError>;

#[derive(Debug, Error)]
pub enum TeaError {
    #[error("failed to create blob from array")]
    FailedBlobCreation,
    #[error("failed to create download link on document")]
    FailedLinkCreation,
    #[error("failed to create object url from blob")]
    FailedObjectUrlCreation,
    #[error("no file on object under associated key")]
    NoFile,
    #[error("nothing to redo")]
    NoRedo,
    #[error("nothing to undo")]
    NoUndo,
    #[error("operated on a uuid {0} with no associated object")]
    NoUuid(Uuid),
    #[error("playmission error")]
    Playmission {
        #[from]
        source: PlaymissionError, 
    }
}