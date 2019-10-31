//! Material representation in wtvr3d, given a WebGlRenderingContext
//!
//! Materials are responsible of compiling and linking shaders as well as
//! managing WebGlPrograms and their uniforms
//!
//! `Material` represents the WebGlProgram itself and its global uniforms,
//! while `MaterialInstance` can use the same underlying Material with
//! different uniform and buffer values.

use super::uniform::{GlobalUniformLocations, Uniform};
use crate::utils::console_warn;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

/// ## Material
///
/// Representation of a reusable Material base, responsible of a `WebGlProgram`
/// linked from vertex and fragment shaders.  
/// It also encapsulates information about its global (shared) uniforms.
///
pub struct Material {
    /// WebGlProgram for this Material. Computed from vertex and fragment shader at creation time.
    program: WebGlProgram,

    /// if `true`, this Material is opaque (`true` by default), for rendering purposes.
    opaque: bool,

    /// Buffers configuration, with common buffer names and locations.
    attribute_locations: HashMap<String, i32>,

    /// Uniforms shared accross all `MaterialInstance`s sharing this parent material.  
    /// Can be overriden in `MaterialInstance` uniforms if needed.
    shared_uniforms: HashMap<String, Uniform>,

    /// Unique ID set for this material.
    id: String,

    /// Location information for global uniforms like View Projection matrix and lights
    pub global_uniform_locations: GlobalUniformLocations,
}

impl Material {
    /// Constructor using a vertex and fragment shader.  
    /// Immediately compiles the shader. Creation should be done at initialization time.  
    ///
    /// ⚠️ This could fail due to compilation errors, thus returning a `Result`
    pub fn new(
        context: &WebGlRenderingContext,
        vert: &str,
        frag: &str,
        id: &str,
    ) -> Result<Material, String> {
        let vertex = compile_shader(context, WebGlRenderingContext::VERTEX_SHADER, vert)?;
        let fragment = compile_shader(context, WebGlRenderingContext::FRAGMENT_SHADER, frag)?;
        let program = link_program(context, &vertex, &fragment)?;
        Ok(Material {
            program: program,
            opaque: true,
            attribute_locations: HashMap::new(),
            shared_uniforms: HashMap::new(),
            id: id.to_owned(),
            global_uniform_locations: GlobalUniformLocations::new(),
        })
    }

    /// Used by buffers to register new attributes to a material.
    pub fn register_new_attribute_location(
        &mut self,
        context: &WebGlRenderingContext,
        name: &str,
    ) -> () {
        if !self.attribute_locations.contains_key(name) {
            self.attribute_locations.insert(
                name.to_owned(),
                context.get_attrib_location(&self.program, name),
            );
        }
    }

    /// Returns a previously computed attribute location if available.
    pub fn get_attribute_location(&self, name: &str) -> Option<i32> {
        if let Some(loc_option) = self.attribute_locations.get(name) {
            Some(loc_option.clone())
        } else {
            None
        }
    }

    /// Location Lookup for this `Material`'s `shared_uniforms`  
    /// This should be called at initialization time.
    pub fn lookup_locations(&mut self, context: &WebGlRenderingContext) -> () {
        self.global_uniform_locations
            .lookup_locations(context, &self.program);
        for (_, uniform) in &mut self.shared_uniforms {
            uniform.lookup_location(context, &self.program);
        }
    }

    /// `self.opaque` setter. Use if your `Material` is semi-transparent.
    pub fn set_transparent(&mut self, transparent: bool) -> () {
        self.opaque = !transparent;
    }

    /// `self.opaque` getter.  
    /// Returns true if the `Material` is semi-transparent.
    pub fn is_transparent(&self) -> bool {
        !self.opaque
    }

    /// Checks if this material has a given `Uniform` depending on its name.
    pub fn has_uniform(&self, name: &str) -> bool {
        self.shared_uniforms.contains_key(name)
    }

    /// Adds a new set of `Uniform`s to the list of uniforms, as a batch.  
    /// Every `Uniform` present in the `WebGlProgram` have to be added before
    /// any rendering step.
    pub fn push_uniforms(&mut self, uniforms: Vec<Uniform>) -> () {
        for uniform in uniforms {
            self.shared_uniforms.insert(uniform.name.clone(), uniform);
        }
    }

    /// Adds a new `Uniform` to the list of uniforms or replaces one with a new value.
    pub fn set_uniform(&mut self, uniform_to_set: Uniform) {
        self.shared_uniforms
            .insert(uniform_to_set.name.clone(), uniform_to_set);
    }

    /// Updates the context with all of this material's uniform.  
    /// Should be called before rendering objects using this material.
    pub fn set_uniforms_to_context(&self, context: &WebGlRenderingContext) -> Result<(), String> {
        for (_, uniform) in &self.shared_uniforms {
            uniform.set_to_context(context).unwrap_or_else(|message| {
                console_warn(&message[..]);
            });
        }
        Ok(())
    }

    /// Returns a reference to this `Material`'s underlying `WebGlProgram`.
    pub fn get_program(&self) -> &WebGlProgram {
        &self.program
    }

    /// Getter for the private `id` attribute.
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

/// ## `MaterialInstance`
///
/// A Mesh-specific material instance. While `Material` is meant to be shared,
/// a `MaterialInstance` should be created for each different mesh.
///
/// Its `uniforms` field lets you override the parent material's shared uniforms,
/// or add instance-specific ones that are not meant to be shared between meshes.
pub struct MaterialInstance {
    /// Parent material shared reference.
    parent_material: Rc<RefCell<Material>>,

    /// Instance-specific map of `Uniform`s.
    uniforms: HashMap<String, Uniform>,

    /// Unique ID for this material instance
    id: String,
}

impl MaterialInstance {
    /// Constructor, taking a `Rc<RefCell<Material>>` as a parent.
    pub fn new(parent_material: Rc<RefCell<Material>>, id: &str) -> MaterialInstance {
        MaterialInstance {
            parent_material: parent_material,
            uniforms: HashMap::new(),
            id: id.to_owned(),
        }
    }

    /// Lookup locations for this `MaterialInstance`.  
    /// If locations are missing from the parent material, they will be computed
    /// automatically.
    pub fn lookup_locations(&mut self, context: &WebGlRenderingContext) -> () {
        let mut parent_mat = self.parent_material.borrow_mut();
        parent_mat.lookup_locations(context);
        for (_, uniform) in &mut self.uniforms {
            uniform.lookup_location(context, parent_mat.get_program());
        }
    }

    /// Adds a new set of `Uniform`s to this `MaterialInstance`, as a batch.  
    /// All necessary `Uniform`s that are present in the shader programs
    /// should be added before rendering.
    pub fn push_uniforms(&mut self, uniforms: Vec<Uniform>) -> () {
        for uniform in uniforms {
            self.set_uniform(uniform);
        }
    }

    /// Getter for the parent's `Material` transparency setting.
    pub fn is_transparent(&self) -> bool {
        self.parent_material.borrow().is_transparent()
    }

    /// Adds or update a mesh-specific `Uniform`.
    pub fn set_uniform(&mut self, uniform_to_set: Uniform) {
        self.uniforms
            .insert(uniform_to_set.name.to_owned(), uniform_to_set);
    }

    /// Updates a global `Uniform` from this `MaterialInstance`'s parent `Material`.
    pub fn set_parent_uniform(&mut self, uniform_to_set: Uniform) {
        let mut parent_mat = self.parent_material.borrow_mut();
        parent_mat.set_uniform(uniform_to_set);
    }

    /// Returns a reference to this `MaterialInstance`'s parent `Rc`
    pub fn get_parent(&self) -> &Rc<RefCell<Material>> {
        &self.parent_material
    }

    /// Getter for id
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Returns the id of this `MaterialInstance`'s parent for sorting purposes.
    pub fn get_parent_id(&self) -> String {
        self.parent_material.borrow().get_id().to_owned()
    }

    /// Updates the context with all of this material's uniform, not including the parent
    /// `Material`'s `Uniform`s.   
    /// Should be called before rendering the Mesh using this `MaterialInstance`.  
    /// ⚠️ The parent's `Uniforms` should be set before that step.
    pub fn set_uniforms_to_context(&self, context: &WebGlRenderingContext) -> Result<(), String> {
        for (_, uniform) in &self.uniforms {
            uniform.set_to_context(context).unwrap_or_else(|message| {
                console_warn(&message[..]);
            });
        }
        Ok(())
    }
}

/// Boilerplate shader compilation function taken from the `wasm-bindgen` WebGL example.
fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let err = Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")));
        context.delete_shader(Some(&shader));
        err
    }
}

/// Boilerplate program linking function taken from the `wasm-bindgen` WebGL example.
fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let err = Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")));
        context.delete_program(Some(&program));
        err
    }
}
