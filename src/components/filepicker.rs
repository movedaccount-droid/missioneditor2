use dioxus::prelude::*;

#[component]
pub fn FilePicker(signal: Signal<File>) -> Element {
    let onchange = move |formdata: Event<FormData>| async move {
        let Some(files) = formdata.data.files() else {
            *signal.write() = File::None;
            return;
        };

        let name = files
            .files()
            .first()
            .expect("formdata contained Some(files) but was empty anyway")
            .to_string();

        *signal.write() = File::Loading;
        let Some(data) = files.read_file(&name).await else {
            *signal.write() = File::None;
            return;
        };

        *signal.write() = File::Loaded { name, data };
    };

    match *signal.read() {
        File::Loading => {
            rsx!(
                p {
                    "loading"
                }
            )
        }
        _ => {
            rsx!(input {
                r#type: "file",
                onchange: onchange
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum File {
    None,
    Loading,
    Loaded { name: String, data: Vec<u8> },
}

impl File {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self::Loaded{ name, data }
    }
}