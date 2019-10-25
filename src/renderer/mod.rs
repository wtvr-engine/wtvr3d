//! # Renderer
//!
//! Rendering Engine for wtvr3d. Uses WebGL through `web-sys`

pub mod material;

pub mod uniform;

use crate::mesh::Mesh;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub struct Renderer<'a> {
    object_repository: HashMap<u32, Vec<Rc<RefCell<Mesh<'a>>>>>,
    webgl_context: WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    next_material_id: u32,
}

impl<'a> Renderer<'a> {
    pub fn new(canvas_id: &str) -> Renderer {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();
        Renderer {
            object_repository: HashMap::new(),
            webgl_context: context,
            canvas: canvas,
            next_material_id: 0,
        }
    }

    pub fn register_mesh(&mut self, mesh: &Rc<RefCell<Mesh<'a>>>) -> () {
        let id = mesh
            .borrow_mut()
            .material
            .get_parent_id(self.next_material_id);
        if self.object_repository.contains_key(&id) {
            let vec = self.object_repository.get_mut(&id).unwrap();
            vec.push(Rc::clone(mesh));
        } else {
            self.object_repository.insert(id, vec![Rc::clone(mesh)]);
        }
    }
}
