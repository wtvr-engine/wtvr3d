//! Rendering Engine for wtvr3d. Uses WebGL through the `web-sys` crate.

mod material;

mod uniform;

mod buffer;

mod shader_data_type;

mod mesh_data;

pub use material::{Material, MaterialInstance};
pub use mesh_data::MeshData;
pub use uniform::{Uniform,UniformValue,GlobalUniformLocations};
pub use buffer::Buffer;
pub use shader_data_type::ShaderDataType;

use crate::asset::AssetRegistry;
use crate::system::LightRepository;
use crate::component::{Camera, Transform};
use crate::scene::FileType;
use crate::utils::console_error;
use nalgebra::Matrix4;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub type SortedMeshes<'a> = HashMap<&'a str, HashMap<&'a str, Vec<(&'a str, &'a Transform)>>>;

/// ## Renderer
///
/// Renderer for `wtvr3D`. Renders meshes from the point of view of a `Camera`  
/// Every Mesh must be registered with the `Renderer` before being rendered.
/// Otherwise, the mesh won't be included in the render.
///
/// A Renderer needs a `WebGlRenderingContext` to render to, and a reference to the
/// associated `HtmlCanvasElement`.
pub struct Renderer {
    /// The current WebGlRenderingContext to render to.
    webgl_context: WebGlRenderingContext,

    /// The target `HtmlCanvasElement`
    canvas: HtmlCanvasElement,

    /// Camera reference used for rendering.
    main_camera: Rc<RefCell<Camera>>,

    /// Asset registry instance for use with this renderer
    asset_registry: AssetRegistry,
}

impl Renderer {
    /// Constructor. Must be provided a Canvas reference, a `WebGlRenderingContext` and a
    /// valid Camera to be used to render the scene.
    pub fn new(
        camera: Camera,
        canvas: HtmlCanvasElement,
        context: WebGlRenderingContext,
    ) -> Renderer {
        Renderer {
            webgl_context: context,
            canvas: canvas,
            main_camera: Rc::new(RefCell::new(camera)),
            asset_registry: AssetRegistry::new(),
        }
    }

    pub fn get_webgl_context(&self) -> &WebGlRenderingContext {
        &self.webgl_context
    }

    /// Resizes the canvas internal size to match the display resolution and ratio.  
    /// Also updates the WebGl Viewport to match.
    ///
    /// ⚠️ might be removed in favor of all-JS version.
    pub fn resize_canvas(&mut self) -> () {
        let display_width = self.canvas.client_width() as u32;
        let display_height = self.canvas.client_height() as u32;
        if self.canvas.width() != display_width || self.canvas.height() != display_height {
            self.canvas.set_width(display_width);
            self.canvas.set_height(display_height);
        }
        let ratio = display_width as f32 / display_height as f32;
        self.main_camera.borrow_mut().set_aspect_ratio(ratio);
        self.webgl_context
            .viewport(0, 0, display_width as i32, display_height as i32);
        self.main_camera.borrow_mut().set_aspect_ratio(ratio)
    }

    /// Renders all the objects registered in the Mesh Repository and prints them to the Canvas.component
    ///
    /// The opaque objects will be rendered before the transparent ones (ordered by depth), and every object will be sorted
    /// by `Material` id to optimize performance.
    // ⭕ TODO handle semi-transparent objects separately
    pub fn render_objects(&self, sorted_meshes: SortedMeshes, light_repository : &LightRepository) {
        let camera = self.main_camera.borrow();
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix();
        self.webgl_context.clear_color(0., 0., 0., 0.);
        self.webgl_context.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        self.webgl_context.enable(WebGlRenderingContext::CULL_FACE);
        self.webgl_context.enable(WebGlRenderingContext::DEPTH_TEST);
        for (material_id, mesh_hash_map) in sorted_meshes {
            self.draw_meshes_using_material(&material_id, mesh_hash_map, view_matrix, projection_matrix, light_repository);
        }
    }

    fn draw_meshes_using_material(
        &self,
        material_id: &str,
        mesh_hash_map: HashMap<&str, Vec<(&str, &Transform)>>,
        view_matrix: &Matrix4<f32>,
        projection_matrix: &Matrix4<f32>,
        light_repository : &LightRepository,
    ) {
        if let Some(material) = self.asset_registry.get_material(&material_id) {
            self.webgl_context
                .use_program(Some(material.borrow().get_program()));
            material
                .borrow()
                .set_uniforms_to_context(&self.webgl_context)
                .ok();
            self.set_camera_uniforms(material.clone(), view_matrix.clone(),projection_matrix.clone())
                .ok();
            for (mesh_data_id, transforms) in mesh_hash_map {
                self.draw_meshes_using_mesh_data(&mesh_data_id, material.clone(), transforms);
            }
        } else {
            console_error(&format!(
                "Meshes were not rendered because material {} is not registered.",
                &material_id
            ));
        }
    }

    fn draw_meshes_using_mesh_data(
        &self,
        mesh_data_id: &str,
        material: Rc<RefCell<Material>>,
        mut transforms: Vec<(&str, &Transform)>,
    ) {
        transforms.sort_by(|a, b| a.0.cmp(b.0));
        let current_mat_instance_id = "";
        if let Some(mesh_data) = self.asset_registry.get_mesh_data(&mesh_data_id) {
            for buffer in mesh_data.get_buffers() {
                let location = material
                    .borrow()
                    .get_attribute_location(buffer.get_attribute_name());
                if let Some(loc) = location {
                    buffer.enable_and_bind_attribute(&self.webgl_context, loc);
                } else {
                    console_error("Could not bind some buffers because locations were missing.");
                }
            }
            for (material_instance_id, transform) in transforms {
                if material_instance_id != current_mat_instance_id {
                    if let Some(material_instance) = self
                        .asset_registry
                        .get_material_instance(material_instance_id)
                    {
                        material_instance
                            .borrow()
                            .set_uniforms_to_context(&self.webgl_context)
                            .ok();
                        self.set_transform_uniform(material.clone(), transform).ok();
                        self.webgl_context.draw_arrays(
                            WebGlRenderingContext::TRIANGLES,
                            0,
                            mesh_data.get_vertex_count(),
                        );
                    } else {
                        console_error(&format!("Meshes were not rendered because material instance {} is not registered.",&material_instance_id));
                    }
                }
            }
        } else {
            console_error(&format!(
                "Meshes were not rendered because mesh_data {} is not registered.",
                &mesh_data_id
            ));
        }
    }

    /// Sets the global camera uniform for the whole scene  
    /// Meant to be used by `Self.render_objects`
    fn set_camera_uniforms(
        &self,
        material: Rc<RefCell<Material>>,
        view_matrix: Matrix4<f32>,
        projection_matrix: Matrix4<f32>,
    ) -> Result<(), String> {
        let camera_view_uniform_location = material
            .borrow_mut()
            .global_uniform_locations
            .view_matrix_location
            .clone();
        let camera_projection_uniform_location = material
            .borrow_mut()
            .global_uniform_locations
            .projection_matrix_location
            .clone();
        let view_matrix_uniform = Uniform::new_with_location(
            uniform::VIEW_MATRIX_NAME,
            camera_view_uniform_location,
            Box::new(view_matrix),
        );
        let projection_matrix_uniform = Uniform::new_with_location(
            uniform::PROJECTION_MATRIX_NAME,
            camera_projection_uniform_location,
            Box::new(projection_matrix),
        );
        view_matrix_uniform.set_to_context(&self.webgl_context)?;
        projection_matrix_uniform.set_to_context(&self.webgl_context)
    }

    /// Sets the world transform uniform for a specific object
    /// Meant to be used by `Self.render_objects`
    fn set_transform_uniform(
        &self,
        material: Rc<RefCell<Material>>,
        transform: &Transform,
    ) -> Result<(), String> {
        let transfom_matrix_location = material
            .borrow_mut()
            .global_uniform_locations
            .world_transform_location
            .clone();
        let world_matrix = transform.get_world_matrix();
        let transform_uniform = Uniform::new_with_location(
            uniform::WORLD_TRANSFORM_NAME,
            transfom_matrix_location,
            Box::new(world_matrix.clone()),
        );
        transform_uniform.set_to_context(&self.webgl_context)
    }

    /// Getter for the asset registry, immutable version
    pub fn get_asset_registry(&self) -> &AssetRegistry {
        &self.asset_registry
    }

    /// Register an asset to the AssetRegistry associated with this Renderer
    pub fn register_asset(
        &mut self,
        file_data: &[u8],
        file_type: FileType,
    ) -> Result<String, String> {
        match file_type {
            FileType::WMesh => self
                .asset_registry
                .register_mesh_data(&self.webgl_context, file_data),
            FileType::WMaterial => self
                .asset_registry
                .register_material(&self.webgl_context, file_data),
            FileType::WMatInstance => self
                .asset_registry
                .register_material_instance(&self.webgl_context, file_data),
        }
    }
}
