use dioxus::prelude::*;
use gloo_console::log;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::three::{ BoxGeometry, Mesh, MeshBasicMaterial, Object3D, OrbitControls, PerspectiveCamera, Scene, WebGLRenderer };
use super::Picker;

#[component]
pub fn Viewport() -> Element {

	let scene: Signal<Option<Scene>> = use_signal(|| None);
    let ren = use_signal(|| None);

    if (*scene.read()).is_none() {
        log!("none");
        init(scene, ren)
    } else {
        log!("some");
    }

    let outer_html = use_memo(move || ren.with( |r: &Option<WebGLRenderer>| {
        if let Some(r) = r {
            r.dom_element().outer_html()
        } else {
            "loading".to_string()
        }
    }));

    rsx! {
        div {
        	"{outer_html}"
        }
    }

}

fn init(mut scene_signal: Signal<Option<Scene>>, mut ren: Scene<Option<WebGLRenderer>>) {

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

    ren.set(WebGLRenderer::new());
    (*ren.write()).set_size(500, 500, true);

    // let mut picker = Picker::new(ren.dom_element());
    let controls = OrbitControls::new(&cam ,(*ren.read()).dom_element());

    (*ren.write()).render(&scene, &cam);
    *scene_signal.write() = Some(scene);

    // Interval::new(16, move || {

    //     let Some(ref scene) = *scene_signal.read() else {
    //         return;
    //     };

    //     let r = cube.rotation();
    //     r.set_x(r.x() + 0.1);
    //     // picker.pick(&scene, &cam);
    //     controls.update();
    //     ren.render(&scene, &cam);

    // })
    // .forget();

}