//! Material asset definition, with implementation.
//!
//! A Material represents a WebGL Program alongside Uniform and Buffer locations.

use serde::{Deserialize, Serialize};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::error::W3DError;

use super::{constructible::Constructible, file::File};

#[cfg(feature = "auto_material")]
use crate::util::{Matches, RegExp};

/// Enum for Shader value types as used in GLSL.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum ShaderValueType {
    Bool,
    Int,
    Float,
    Double,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
    Sampler2D,
    Unimplemented,
}

impl ShaderValueType {
    #[cfg(feature = "auto_material")]
    pub fn from_str(text: &str) -> ShaderValueType {
        match text {
            "bool" => ShaderValueType::Bool,
            "int" => ShaderValueType::Int,
            "float" => ShaderValueType::Float,
            "double" => ShaderValueType::Double,
            "vec2" => ShaderValueType::Vec2,
            "vec3" => ShaderValueType::Vec3,
            "vec4" => ShaderValueType::Vec4,
            "mat2" => ShaderValueType::Mat2,
            "mat3" => ShaderValueType::Mat3,
            "mat4" => ShaderValueType::Mat4,
            "sampler2D" => ShaderValueType::Sampler2D,
            _ => ShaderValueType::Unimplemented,
        }
    }
}

/// Struct representing a shader attribute or uniform definition
#[derive(Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub name: String,
    pub value_type: ShaderValueType,
    #[serde(skip)]
    pub location: Option<i32>,
}

/// Struct representing a shader attribute or uniform definition
#[derive(Serialize, Deserialize, Clone)]
pub struct Uniform {
    pub name: String,
    pub value_type: ShaderValueType,
    #[serde(skip)]
    pub location: Option<WebGlUniformLocation>,
}

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

    /// Attribute names for location lookup
    attributes: Vec<Attribute>,

    /// Uniform names for location lookup
    uniforms: Vec<Uniform>,
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
            attributes: Vec::new(),
            uniforms: Vec::new(),
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

    fn get_attrib_locations(&mut self, context: &WebGl2RenderingContext) -> Result<(), W3DError> {
        match &self.program {
            Some(p) => {
                for attribute in &mut self.attributes {
                    let location = context.get_attrib_location(p, &attribute.name);
                    if location == -1 {
                        return Err(W3DError::new(
                            "Attribute Location was not found",
                            Some(self.name.clone()),
                        ));
                    } else {
                        attribute.location = Some(location);
                    }
                }
                Ok(())
            }
            _ => Err(W3DError::new(
                "Trying to get attribute location without program",
                Some(self.name.clone()),
            )),
        }
    }

    fn get_uniform_locations(&mut self, context: &WebGl2RenderingContext) -> Result<(), W3DError> {
        match &self.program {
            Some(p) => {
                for uniform in &mut self.uniforms {
                    let location = context.get_uniform_location(p, &uniform.name);
                    if let Some(loc) = location {
                        uniform.location = Some(loc);
                    } else {
                        return Err(W3DError::new(
                            "Uniform Location was not found",
                            Some(self.name.clone()),
                        ));
                    }
                }
                Ok(())
            }
            _ => Err(W3DError::new(
                "Trying to get uniform location without program",
                Some(self.name.clone()),
            )),
        }
    }

    fn get_locations(&mut self, context: &WebGl2RenderingContext) -> Result<(), W3DError> {
        self.get_attrib_locations(context)?;
        self.get_uniform_locations(context)?;
        Ok(())
    }

    #[cfg(feature = "auto_material")]
    fn set_attribute_and_uniform_names(&mut self) {
        let attribute_re = RegExp::new(r"in (.*) (.*);");
        let uniform_re = RegExp::new(r"uniform (.*) (.*);");

        if self.attributes.len() > 0 || self.uniforms.len() > 0 {
            return;
        }
        if let (Some(v_shader), Some(f_shader)) = (&self.vertex_shader, &self.fragment_shader) {
            for matches in attribute_re.exec(v_shader) {
                self.attributes.push(Attribute {
                    name: matches.groups[1].clone(),
                    value_type: ShaderValueType::from_str(&matches.groups[0]),
                    location: None,
                });
            }
            for matches in uniform_re.exec(v_shader) {
                self.uniforms.push(Uniform {
                    name: matches.groups[1].clone(),
                    value_type: ShaderValueType::from_str(&matches.groups[0]),
                    location: None,
                });
            }
            for matches in uniform_re.exec(f_shader) {
                self.uniforms.push(Uniform {
                    name: matches.groups[1].clone(),
                    value_type: ShaderValueType::from_str(&matches.groups[0]),
                    location: None,
                });
            }
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

                #[cfg(feature = "auto_material")]
                self.set_attribute_and_uniform_names();

                self.get_locations(context)?;

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
