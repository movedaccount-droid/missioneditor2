use dioxus::prelude::*;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;

use crate::three::{ BoxGeometry, Mesh, MeshBasicMaterial, Object3D, OrbitControls, PerspectiveCamera, Scene, WebGLRenderer };
use super::Picker;

#[component]
pub fn Viewport() -> Element {

    // fffuckkk offf https://stackoverflow.com/questions/34863788/how-to-check-if-an-element-has-been-loaded-on-a-page-before-running-a-script
    // was possible in 0.4.3 natively https://docs.rs/dioxus-hooks/0.4.3/dioxus_hooks/fn.use_effect.html
    rsx! {
        div {
            id: "viewport-container",
            iframe {
                display: "none",
                width: 0,
                height: 0,
                onload: move |_| { init(); }
            }
        }
    }

}

// after the page has been rendered and we have a container,
// load the actual [static-lifetime] viewport to it
fn init() {

    let container = web_sys::window().unwrap()
        .document().unwrap()
        .get_element_by_id("viewport-container").unwrap();

    let scene = Scene::new();

    let cam = PerspectiveCamera::new(75.0, 1.0, 0.1, 1000.0);
    cam.position().set(0.0, 0.0, 5.0);

    let geo = BoxGeometry::new(1.0, 1.0, 1.0);
    let mat = MeshBasicMaterial::new();
    mat.color().set_rgb(1.0, 0.0, 0.0);
    let cube = Mesh::new(&geo, &mat);

    cube.dyn_ref::<Object3D>()
        .unwrap()
        .set_name(String::from("cuuubed out the fucking . head"));

    scene.add(&cube);

    let ren = WebGLRenderer::new();
    ren.set_size(500, 500, true);

    let mut picker = Picker::new(ren.dom_element());
    let controls = OrbitControls::new(&cam, &ren.dom_element());

    // TODO: fix unwrap... although tihs shiould never fail
    container.append_child(&ren.dom_element()).unwrap();
    ren.render(&scene, &cam);

    Interval::new(16, move || {

        let r = cube.rotation();
        r.set_x(r.x() + 0.1);
        picker.pick(&scene, &cam);
        controls.update();
        ren.render(&scene, &cam);

    })
    .forget();

}