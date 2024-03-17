use dioxus::{html::canvas, prelude::*};
use gloo_console::log;
use gloo_timers::callback::Interval;
use uuid::Uuid;
use wasm_bindgen::JsCast;

use crate::three::{ BoxGeometry, Mesh, MeshBasicMaterial, Object3D, OrbitControls, PerspectiveCamera, Scene, WebGLRenderer };
use super::Picker;

#[component]
pub fn Viewport(scene_signal: Signal<Option<Scene>>, selected_signal: Signal<Uuid>) -> Element {

    // fffuckkk offf https://stackoverflow.com/questions/34863788/how-to-check-if-an-element-has-been-loaded-on-a-page-before-running-a-script
    // was possible in 0.4.3 natively https://docs.rs/dioxus-hooks/0.4.3/dioxus_hooks/fn.use_effect.html
    rsx! {
        div {
            id: "viewport-container",
            iframe {
                display: "none",
                width: 0,
                height: 0,
                onload: move |_| { init(scene_signal, selected_signal); }
            }
        }
    }

}

// after the page has been rendered and we have a container,
// load the actual [static-lifetime] viewport to it
fn init(mut scene_signal: Signal<Option<Scene>>, mut selected_signal: Signal<Uuid>) {

    let container = web_sys::window().unwrap()
        .document().unwrap()
        .get_element_by_id("viewport-container").unwrap();

    let scene = Scene::new();

    let cam = PerspectiveCamera::new(75.0, 1.0, 0.1, 1000.0);
    cam.position().set(0.0, 0.0, 5.0);

    let ren = WebGLRenderer::new();
    ren.set_size(500, 500, true);

    let mut picker = Picker::new(ren.dom_element());
    let controls = OrbitControls::new(&cam, &ren.dom_element());

    // TODO: fix unwrap... although tihs shiould never fail
    container.append_child(&ren.dom_element()).unwrap();
    ren.render(&scene, &cam);

    Interval::new(16, move || {

        if let Some(selected) = picker.pick(scene_signal.write().iter_mut().next().expect("FAILED_ONE"), &cam) {
            *selected_signal.write() = selected;
        }
        controls.update();
        ren.render(scene_signal.write().iter_mut().next().expect("FAILED_TWO"), &cam);

    })
    .forget();

    *scene_signal.write() = Some(scene);

}