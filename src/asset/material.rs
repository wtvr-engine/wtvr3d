//! Material asset definition, with implementation.
//!
//! A Material represents a WebGL Program alongside Uniform and Buffer locations.

use serde::{Deserialize, Serialize};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use crate::error::W3DError;

use super::{constructible::Constructible, file::File};

/// # Material struct
/// A Material represents a WebGL Program alongside Uniform and Buffer locations.
///
/// When constructed, it will compile its internal shaders and discard the text versions.
///
/// It is meant to be used by a Renderer.
#[derive(Serialize, Deserialize)]
pub struct Material {
    /// Underlying program. Can be None until constructed.
    #[serde(skip)]
    program: Option<WebGlProgram>,

    /// Vertex shader text
    vertex_shader: Option<String>,

    /// Fragment shader text
    fragment_shader: Option<String>,

    // TODO : use this
    /// Is this material using lights ?
    lit: bool,

    // TODO : use this
    /// Is this material semi-transparent ?
    transparent: bool,

    /// Identification of this material for easy error handling
    name: String,
}

impl Material {
    /// Create a new material from vertex and fragment shaders.
    /// Shader strings will be dropped when the shader is constructed.
    pub fn new(
        name: String,
        vertex_shader: String,
        fragment_shader: String,
        lit: bool,
        transparent: bool,
    ) -> Material {
        Material {
            name,
            program: None,
            vertex_shader: Some(vertex_shader),
            fragment_shader: Some(fragment_shader),
            lit,
            transparent,
        }
    }

    pub fn get_program(&self) -> Option<&WebGlProgram> {
        self.program.as_ref()
    }

    fn compile_shader(
        &self,
        shader_text: &str,
        shader_type: u32,
        context: &WebGl2RenderingContext,
    ) -> Result<WebGlShader, W3DError> {
        let shader = context.create_shader(shader_type).ok_or_else(|| {
            W3DError::new("Shader could not be created.", Some(self.name.clone()))
        })?;
        context.shader_source(&shader, shader_text);
        context.compile_shader(&shader);
        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .is_truthy()
        {
            Ok(shader)
        } else {
            let log = context.get_shader_info_log(&shader);
            context.delete_shader(Some(&shader));
            Err(W3DError::new_with_desc(
                "Compilation failed for shader",
                Some(self.name.clone()),
                log,
            ))
        }
    }

    fn link_program(
        &self,
        v_shader: &WebGlShader,
        f_shader: &WebGlShader,
        context: &WebGl2RenderingContext,
    ) -> Result<WebGlProgram, W3DError> {
        let program = context.create_program().ok_or_else(|| {
            W3DError::new("Could not create WebGL Program", Some(self.name.clone()))
        })?;
        context.attach_shader(&program, v_shader);
        context.attach_shader(&program, f_shader);
        context.link_program(&program);
        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .is_truthy()
        {
            Ok(program)
        } else {
            let log = context.get_program_info_log(&program);
            context.delete_program(Some(&program));
            Err(W3DError::new_with_desc(
                "Linking failed for WebGLProgram",
                Some(self.name.clone()),
                log,
            ))
        }
    }
}

impl Constructible for Material {
    fn construct(
        &mut self,
        context: &WebGl2RenderingContext,
        clean_up: bool,
    ) -> Result<(), W3DError> {
        match (&self.vertex_shader, &self.fragment_shader) {
            (Some(v_shader_text), Some(f_shader_text)) => {
                let v_shader = self.compile_shader(
                    v_shader_text,
                    WebGl2RenderingContext::VERTEX_SHADER,
                    context,
                )?;
                let f_shader = self.compile_shader(
                    f_shader_text,
                    WebGl2RenderingContext::FRAGMENT_SHADER,
                    context,
                )?;
                let program = self.link_program(&v_shader, &f_shader, context)?;
                self.program = Some(program);
                if clean_up {
                    self.vertex_shader = None;
                    self.fragment_shader = None;
                }
                Ok(())
            }
            _ => Err(W3DError::new(
                "Missing shader for material",
                Some(self.name.clone()),
            )),
        }
    }

    fn is_constructed(&self) -> bool {
        self.program.is_some()
    }
}

impl<'a> File<'a> for Material {
    fn get_name(&self) -> std::string::String {
        self.name.clone()
    }
}
