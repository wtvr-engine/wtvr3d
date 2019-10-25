//! # Uniform
//! 
//! Interface and implementations for managing uniforms

use std::slice;
use web_sys::{WebGlRenderingContext, WebGlUniformLocation, WebGlProgram};
use nalgebra::base::{Vector2,Vector3,Vector4,Matrix2,Matrix3,Matrix4};

pub enum UniformType {
    Single,
    Vector2,
    Vector3,
    Vector4,
    Matrix2,
    Matrix3,
    Matrix4,
    Sampler2D,
}

pub struct Uniform<'a> {
    pub name : &'a str,
    location : Option<WebGlUniformLocation>,
    pub value : Box<dyn UniformValue>,
}

impl<'a> Uniform<'a> {
    pub fn new(name : &'a str, value : Box<dyn UniformValue>) -> Uniform {
        Uniform {
            name : name,
            location : None,
            value : value,
        }
    }

    pub fn lookup_location(&mut self, context : &WebGlRenderingContext, program : &WebGlProgram) -> () {
        if self.location == None {
            self.location = context.get_uniform_location(program,self.name)
        }
    }

    pub fn set(&self, context : &WebGlRenderingContext) -> Result<(),String> {
        let result = self.value.set_uniform(context, if let Some(loc) = &self.location {Some(&loc)} else {None});
        if let Err(message) = result {
            Err(format!("Uniform {} couldn't be set; {}",self.name,message).to_string())
        } else{
            result
        }
    }
}

pub trait UniformValue {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String>;
}

impl UniformValue for f32 {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        context.uniform1fv_with_f32_array(location,slice::from_ref(self));
        Ok(())
    }
}


impl UniformValue for &[f32] {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        (UniformType::Single,*self).set_uniform(context,location)
    }
}

impl UniformValue for (UniformType,&[f32]) {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
       match self.0 {
           UniformType::Single => { context.uniform1fv_with_f32_array(location,self.1); Ok(())},
           UniformType::Vector2 => { context.uniform2fv_with_f32_array(location,self.1); Ok(())},
           UniformType::Vector3 => { context.uniform3fv_with_f32_array(location,self.1); Ok(())},
           UniformType::Vector4 => { context.uniform4fv_with_f32_array(location,self.1); Ok(())},
           UniformType::Matrix2 => { context.uniform_matrix2fv_with_f32_array(location,false,self.1); Ok(())},
           UniformType::Matrix3 => { context.uniform_matrix3fv_with_f32_array(location,false,self.1); Ok(())},
           UniformType::Matrix4 => { context.uniform_matrix4fv_with_f32_array(location,false,self.1); Ok(())},
           _ => Err(String::from("Invalid value supplied to uniform"))
       }
    }
}

impl UniformValue for i32 {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        context.uniform1iv_with_i32_array(location,slice::from_ref(self));
        Ok(())
    }
}

impl UniformValue for &[i32] {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        (UniformType::Single,*self).set_uniform(context,location)
    }
}

impl UniformValue for (UniformType,&[i32]){
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
       match self.0 {
           UniformType::Single => { context.uniform1iv_with_i32_array(location,self.1); Ok(())},
           UniformType::Vector2 => { context.uniform2iv_with_i32_array(location,self.1); Ok(())},
           UniformType::Vector3 => { context.uniform3iv_with_i32_array(location,self.1); Ok(())},
           UniformType::Vector4 => { context.uniform4iv_with_i32_array(location,self.1); Ok(())},
           _ => Err(String::from("Invalid value supplied to uniform"))
       }
    }
}

impl UniformValue for Vector2<f32> {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        (UniformType::Vector2,self.as_slice()).set_uniform(context,location)
    }
}

impl UniformValue for &[Vector2<f32>] {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        let mut vec : Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(),vector.as_slice().iter().cloned());
        }
        (UniformType::Vector2,vec.as_slice()).set_uniform(context,location)
    }
}


impl UniformValue for Vector3<f32> {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        (UniformType::Vector3,self.as_slice()).set_uniform(context,location)
    }
}

impl UniformValue for &[Vector3<f32>] {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        let mut vec : Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(),vector.as_slice().iter().cloned());
        }
        (UniformType::Vector3,vec.as_slice()).set_uniform(context,location)
    }
}

impl UniformValue for Vector4<f32> {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        (UniformType::Vector4,self.as_slice()).set_uniform(context,location)
    }
}

impl UniformValue for &[Vector4<f32>] {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        let mut vec : Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(),vector.as_slice().iter().cloned());
        }
        (UniformType::Vector4,vec.as_slice()).set_uniform(context,location)
    }
}

impl UniformValue for Matrix2<f32> {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        
        (UniformType::Matrix2,self.as_slice()).set_uniform(context,location)
    }
}
impl UniformValue for Matrix3<f32> {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        (UniformType::Matrix3,self.as_slice()).set_uniform(context,location)
    }
}
impl UniformValue for Matrix4<f32> {
    fn set_uniform(&self, context : &WebGlRenderingContext, location : Option<&WebGlUniformLocation>) -> Result<(),String> {
        (UniformType::Matrix4,self.as_slice()).set_uniform(context,location)
    }
}
