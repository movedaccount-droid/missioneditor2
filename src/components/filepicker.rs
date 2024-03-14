use dioxus::prelude::*;

#[component]
pub fn FilePicker(signal: Signal<File>) -> Element {

    let onchange = move |formdata: Event<FormData>| async move {
        let Some(files) = formdata.data.files() else {
            signal.set(File::None);
            return;
        };

        let name = files
            .files()
            .first()
            .expect("formdata contained Some(files) but was empty anyway")
            .to_string();

        signal.set(File::Loading);
        let Some(data) = files.read_file(&name).await else {
            signal.set(File::None);
            return;
        };

        signal.set(File::Loaded { name, data });
    };

    let loading = if let Ok(f) = signal.try_read() {
        if let File::Loading = *f {
            true
        } else {
            false
        }
    } else {
        false
    };

    if loading {
        rsx!(
            p {
                "loading"
            }
        )
    } else {
        rsx!(input {
            r#type: "file",
            onchange: onchange
        })
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