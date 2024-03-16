use std::{collections::{HashMap, VecDeque}, io::Cursor, ops::{Deref, DerefMut}};

use dioxus::events::keyboard_types::KeyboardEvent;
use dioxus::events::Code;
use uuid::Uuid;

use crate::playmission::{
    error::PlaymissionError, MissionObject, Object, Value
};

// manages The Elm Architecture for interfacing with the inner project
struct TeaHandler {
    missionobject: MissionObject,
    objects: HashMap<Uuid, Object>,
    status: Option<String>,
    undo_buffer: VecDeque<Event>,
    redo_buffer: VecDeque<InverseEvent>,
    saved: Option<Vec<u8>>
}

impl TeaHandler {

    // import a mission from a file
    fn new_from_buffer(data: Vec<u8>) -> Self {
        let cursor = Cursor::new(data);
        let (missionobject, objects) = MissionObject::deserialize(cursor).unwrap();
        Self {
            missionobject,
            objects,
            status: None,
            undo_buffer: VecDeque::new(),
            redo_buffer: VecDeque::new(),
            saved: None
        }
    }

    // entry point to start event chain
    pub fn event(&mut self, event: Event) {
        self.reset_state();
        let result = self.run_event(event);
        if let Err(e) = result {
            self.status = Some(e.to_string());
        }
    }

    // matches and handles events
    fn run_event(&mut self, event: Event) -> UpdateResult {

        match event {
            Event::Save => self.save(),
            _ => Ok(None),
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
        self.saved = Some(buf);
        Ok(None)
    }

    // handles keypresses, i.e. ctrl+z
    fn keypress(&mut self, e: KeyboardEvent) -> UpdateResult {

        if e.code == Code::KeyZ && e.modifiers.ctrl() {
            let event = if e.modifiers.shift() {
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
        let inverse_event = self.run_event(event)?;
        if let Some(inverse) = inverse_event {
            self.push_redo(inverse);
        }
        Ok(None)
    }

    // redoes an event, if available
    fn redo(&mut self) -> UpdateResult {
        let event = self.redo_buffer.pop_front().ok_or(TeaError::NoUndo)?;
        let inverse_event = self.run_event(event.unwrap())?;
        if let Some(inverse) = inverse_event {
            self.push_undo(inverse.unwrap());
        }
        Ok(None)
    }

    // pushes an event to the undo buffer
    fn push_undo(&mut self, event: Event) {
        self.redo_buffer.clear();
        self.undo_buffer.push_front(event);
        self.undo_buffer.truncate(200);
    }

    // pushes inverseevent to redo buffer
    fn push_redo(&mut self, event: InverseEvent) {
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
    pub fn display_objects(&self) -> Vec<(&Uuid, String)> {
        self.objects.iter()
        .map(|(k, v)| (k, v.name().unwrap_or("{unnamed object}".into())))
        .collect()
    }

    // returns (k, v) of property names and values
    pub fn display_properties(&self, uuid: Uuid) -> ViewResult<Vec<(&str, &Value)>> {
        Ok(
            self.get_object(uuid)?
                .properties()
                .iter()
                .map(|(k, v)| (&**k, v.value()))
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
    pub fn display_files(&self, uuid: Uuid) -> ViewResult<Vec<(&str, &[u8])>> {
        Ok(
            self.get_object(uuid)?
                .files()
                .iter()
                .map(|(k, v)| (&**k, &**v))
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

}

pub enum Event {
    Save,
    Keypress{e: KeyboardEvent},
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
enum TeaError {
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