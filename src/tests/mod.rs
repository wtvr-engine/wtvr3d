//! # Test module
//!
//! This module is not compiled by default. Its purpose is to expose test functions.
#[cfg(feature = "debug")]
extern crate console_error_panic_hook;

use crate::component::camera::Camera;
use crate::component::mesh::{Mesh, MeshData};
use crate::renderer::material::{Material, MaterialInstance};
use crate::renderer::shader_data_type::ShaderDataType;
use crate::renderer::{Buffer, Renderer};
use js_sys::{Float32Array};
use nalgebra::Point3;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::console_error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

#[wasm_bindgen]
pub fn simple_mesh(
    cube_vertices: Float32Array,
    cube_colors: Float32Array,
    vertex_shader: &str,
    fragment_shader: &str,
    canvas_id: &str,
) -> () {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    let context = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();
    let camera = Camera::new(
        16. / 9.,
        3.14 / 2.0,
        1.0,
        1000.0,
        &Point3::new(5.0, 3.0, 8.0),
        &Point3::new(0.0, 0.0, 0.0),
    );
    let cube_buffer = Buffer::from_f32_data(
        &context,
        "a_position",
        ShaderDataType::Vector3,
        cube_vertices,
    );
    let color_buffer =
        Buffer::from_f32_data(&context, "a_color", ShaderDataType::Vector3, cube_colors);
    let mut mesh_data = MeshData::new(36);
    mesh_data.push_buffer(cube_buffer);
    mesh_data.push_buffer(color_buffer);
    let material = Material::new(&context, vertex_shader, fragment_shader).unwrap_or_else(|message| {
        console_error(message.as_str());
        std::panic!("Test failed. Material could not be computed.");
    });
    let mat_instance = MaterialInstance::new(Rc::new(RefCell::new(material)));
    let mesh = Mesh::new(mesh_data, mat_instance);
    let mut renderer = Renderer::new(camera, canvas, context);
    renderer.register_mesh(&Rc::new(RefCell::new(mesh)));
    renderer.resize_canvas();
    renderer.render_objects();
}
