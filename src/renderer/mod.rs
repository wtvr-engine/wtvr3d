//! # Renderer
//!
//! Rendering Engine for wtvr3d. Uses WebGL through `web-sys`

pub mod material;

pub mod uniform;

pub mod buffer;

pub mod shader_data_type;

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
        let mut mesh_mut = mesh.borrow_mut();
        let id = mesh_mut.material.get_parent_id(self.next_material_id);
        if self.object_repository.contains_key(&id) {
            let vec = self.object_repository.get_mut(&id).unwrap();
            vec.push(Rc::clone(mesh));
        } else {
            self.object_repository.insert(id, vec![Rc::clone(mesh)]);
        }
        mesh_mut.material.lookup_locations(&self.webgl_context);
    }

    pub fn resize_canvas(&mut self) -> () {
        let display_width = self.canvas.client_width() as u32;
        let display_height = self.canvas.client_height() as u32;
        if self.canvas.width() != display_width || self.canvas.height() != display_height {
            self.canvas.set_width(display_width);
            self.canvas.set_height(display_height);
        }
        self.webgl_context
            .viewport(0, 0, display_width as i32, display_height as i32);
    }

    pub fn render_objects(&self) {
        self.webgl_context.clear_color(0., 0., 0., 0.);
        self.webgl_context.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        let meshes = self.sort_objects();
        let mut current_id = u32::max_value();
        for mesh_rc in meshes {
            let mut mesh = mesh_rc.borrow_mut();
            let material_id = mesh.material.get_parent_id(0);
            if material_id != current_id {
                current_id = material_id;
                self.webgl_context
                    .use_program(Some(mesh.material.get_parent().borrow().get_program()));
            }
            self.draw_mesh(&mesh);
        }
    }

    fn draw_mesh(&self, mesh: &Mesh<'a>) {
        let position_buffer = self.webgl_context.create_buffer().unwrap();
        self.webgl_context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
    }

    fn sort_objects(&self) -> Vec<Rc<RefCell<Mesh<'a>>>> {
        let mut opaque_meshes = Vec::new();
        let mut transparent_meshes = Vec::new();
        for (_, mesh_vec) in &self.object_repository {
            for mesh in mesh_vec {
                if mesh.borrow().material.is_transparent() {
                    transparent_meshes.push(Rc::clone(&mesh));
                } else {
                    opaque_meshes.push(Rc::clone(&mesh));
                }
            }
        }
        // Sort transparent objects depending on depth
        opaque_meshes.append(&mut transparent_meshes);
        opaque_meshes
    }
}
