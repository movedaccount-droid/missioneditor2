use crate::three::{Object3D, PerspectiveCamera, Raycaster, Scene, Vector2};
use gloo_console::log;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

pub struct Picker {
    raycaster: Raycaster,
    pick_position: Rc<RefCell<PickPosition>>,
}

impl Picker {
    pub fn new(inside: web_sys::HtmlCanvasElement) -> Self {
        Self {
            raycaster: Raycaster::new(),
            pick_position: PickPosition::new(inside),
        }
    }

    pub fn pick(&mut self, scene: &Scene, camera: &PerspectiveCamera) {
        self.raycaster
            .set_from_camera(&self.pick_position.borrow().to_vec(), camera);

        let children = scene
            .dyn_ref::<Object3D>()
            .expect("scene could not wrangle to o3d")
            .children();

        let intersected = self.raycaster.intersect_objects(children, false);

        if !intersected.is_empty() {
            let intersected = js_sys::Reflect::get(&intersected[0], &JsValue::from_str("object"))
                .expect("intersected object did not contain 'object'");
            let intersected_o3d = intersected
                .dyn_ref::<Object3D>()
                .expect("intersected object 'object' was not object3d");
            log!(intersected_o3d.name());
        }
    }
}

struct Position {
    x: f64,
    y: f64,
}

struct PickPosition {
    x: f64,
    y: f64,
    inside: web_sys::HtmlCanvasElement,
}

impl PickPosition {
    // TODO: fucjikued up unwraps

    fn new(inside: web_sys::HtmlCanvasElement) -> Rc<RefCell<Self>> {
        let mut new = Self {
            x: 0.0,
            y: 0.0,
            inside,
        };
        new.clear();
        let cell = Rc::new(RefCell::new(new));

        let c = Rc::clone(&cell);
        let cl = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
            c.borrow_mut().set_from_event(&e);
        });
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mousemove", cl.as_ref().unchecked_ref())
            .unwrap();
        cl.forget();

        let c = Rc::clone(&cell);
        let cl = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| c.borrow_mut().clear());
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mouseout", cl.as_ref().unchecked_ref())
            .unwrap();
        cl.forget();

        let c = Rc::clone(&cell);
        let cl = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| c.borrow_mut().clear());
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mouseleave", cl.as_ref().unchecked_ref())
            .unwrap();
        cl.forget();

        cell
    }

    fn set(&self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    fn clear(&mut self) {
        self.x = -100_000.0;
        self.y = -100_000.0;
    }

    fn get_canvas_rel_pos(&self, event: &web_sys::MouseEvent) -> Position {
        let rect = self.inside.get_bounding_client_rect();
        Position {
            x: (f64::from(event.client_x()) - rect.left()) * f64::from(self.inside.width())
                / rect.width(),
            y: (f64::from(event.client_y()) - rect.top()) * f64::from(self.inside.height())
                / rect.height(),
        }
    }

    fn set_from_event(&self, event: &web_sys::MouseEvent) {
        let pos = self.get_canvas_rel_pos(event);
        self.set(
            pos.x / f64::from(self.inside.width()) * 2.0 - 1.0,
            pos.y / f64::from(self.inside.height()) * (-2.0) + 1.0,
        );
    }

    fn to_vec(&self) -> Vector2 {
        Vector2::new(*self.x.borrow(), *self.y.borrow())
    }
}
