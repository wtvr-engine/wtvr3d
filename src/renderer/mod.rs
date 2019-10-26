//! # Renderer
//!
//! Rendering Engine for wtvr3d. Uses WebGL through `web-sys`

pub mod material;

pub mod uniform;

pub mod buffer;

pub use buffer::Buffer;

pub mod shader_data_type;

use crate::component::camera::Camera;
use crate::component::mesh::Mesh;
use nalgebra::Matrix4;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use uniform::Uniform;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub struct Renderer<'a> {
    mesh_repository: HashMap<u32, Vec<Rc<RefCell<Mesh<'a>>>>>,
    webgl_context: WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    next_material_id: u32,
    main_camera: Rc<RefCell<Camera>>,
}

impl<'a> Renderer<'a> {
    pub fn new(
        camera: Camera,
        canvas: HtmlCanvasElement,
        context: WebGlRenderingContext,
    ) -> Renderer<'a> {
        Renderer {
            mesh_repository: HashMap::new(),
            webgl_context: context,
            canvas: canvas,
            next_material_id: 0,
            main_camera: Rc::new(RefCell::new(camera)),
        }
    }

    pub fn register_mesh(&mut self, mesh: &Rc<RefCell<Mesh<'a>>>) -> () {
        let mut mesh_mut = mesh.borrow_mut();
        mesh_mut.lookup_locations(&self.webgl_context);
        let id = mesh_mut.material.get_parent_id(self.next_material_id);
        if self.mesh_repository.contains_key(&id) {
            let vec = self.mesh_repository.get_mut(&id).unwrap();
            vec.push(Rc::clone(mesh));
        } else {
            self.mesh_repository.insert(id, vec![Rc::clone(mesh)]);
        }
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
        self.main_camera
            .borrow_mut()
            .set_aspect_ratio(display_width as f32 / display_height as f32)
    }

    pub fn render_objects(&self) {
        let vp_matrix = self.main_camera.borrow_mut().compute_vp_matrix().clone();
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
                self.set_camera_uniform(&mut mesh, vp_matrix.clone()).ok();
            }
            self.draw_mesh(&mesh);
        }
    }

    fn draw_mesh(&self, mesh: &Mesh<'a>) {
        for buffer in mesh.get_buffers() {
            buffer.enable_and_bind_attribute(&self.webgl_context);
        }
        self.webgl_context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            mesh.get_vertex_count(),
        );
    }

    fn set_camera_uniform(&self, mesh: &mut Mesh, vp_matrix: Matrix4<f32>) -> Result<(), String> {
        let camera_uniform_location = mesh
            .material
            .get_parent()
            .borrow()
            .global_uniform_locations
            .vp_matrix_location
            .clone();
        let vp_matrix_uniform = Uniform::new_with_location(
            uniform::VP_MATRIX_NAME,
            camera_uniform_location,
            Box::new(vp_matrix),
        );
        vp_matrix_uniform.set(&self.webgl_context)
    }

    fn sort_objects(&self) -> Vec<Rc<RefCell<Mesh<'a>>>> {
        let mut opaque_meshes = Vec::new();
        let mut transparent_meshes = Vec::new();
        for (_, mesh_vec) in &self.mesh_repository {
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
