//! # webgl_program
//! 
//! webgl program representation in wtvr3d, given a WebGL context

use std::collections::HashMap;
use std::rc::Rc;
use super::uniform::Uniform;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};


pub struct Material<'a> {
    program : WebGlProgram,
    uniforms : HashMap<&'a str,Uniform<'a>>,
}

impl<'a> Material<'a> {
    pub fn new(context : &WebGlRenderingContext, vert : &str, frag : &str) -> Material<'a> {
        let vertex = compile_shader(context, WebGlRenderingContext::VERTEX_SHADER, vert).unwrap();
        let fragment = compile_shader(context, WebGlRenderingContext::FRAGMENT_SHADER, frag).unwrap();
        let program = link_program(context, &vertex, &fragment).unwrap();
        Material {
            program : program,
            uniforms : HashMap::new(),
        }
    }

    pub fn push_uniforms(&mut self, uniforms : Vec<Uniform<'a>>) -> () {
        for uniform in uniforms {
            self.uniforms.insert(uniform.name, uniform);
        }
    }

    pub fn set_uniform(&mut self, uniform_to_set : Uniform<'a>){
        self.uniforms.insert(uniform_to_set.name,uniform_to_set);
    }

    pub fn set_uniforms_to_context(&self,context : &WebGlRenderingContext) -> Result<(),String> {
        for (_,uniform) in &self.uniforms {
            uniform.set(context)?
        }
        Ok(())
    }
}

pub struct MaterialInstance<'a> {
    parent_material : Rc<Material<'a>>,
    uniforms : HashMap<&'a str,Uniform<'a>>
}

impl<'a> MaterialInstance<'a> {
    pub fn new(parent_material : Rc<Material>) -> MaterialInstance {
        MaterialInstance {
            parent_material : parent_material,
            uniforms : HashMap::new(),
        }
    }
}

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