//! Deserializer for files generated using the wtvr3d Asset Converter
mod asset_registry;

pub use asset_registry::AssetRegistry;

use crate::renderer::buffer::Buffer;
use crate::renderer::material::{Material, MaterialInstance};
use crate::renderer::uniform::{Uniform, UniformValue};
use crate::renderer::MeshData;
use bincode::deserialize;
use web_sys::WebGlRenderingContext;
use wtvr3d_file::{
    FileBuffer, FileValue, MaterialFile, MaterialInstanceFile, MeshFile, ShaderDataType, Triangle,
};

pub fn deserialize_wmesh(context: &WebGlRenderingContext, data: &[u8]) -> Result<MeshData, String> {
    let mesh_files_result = deserialize::<MeshFile>(data);
    match mesh_files_result {
        Err(_) => Err(String::from("Could not deserialize the given mesh file.")),
        Ok(mesh_file) => Ok(make_mesh_data_from(context, &mesh_file)),
    }
}

pub fn deserialize_wmaterial(
    context: &WebGlRenderingContext,
    data: &[u8],
) -> Result<Material, String> {
    let material_files_result = deserialize::<MaterialFile>(data);
    match material_files_result {
        Err(_) => Err(String::from(
            "Could not deserialize the given material file.",
        )),
        Ok(material_file) => make_material_from(context, &material_file),
    }
}

pub fn deserialize_wmatinstance(
    asset_registry: &AssetRegistry,
    data: &[u8],
) -> Result<MaterialInstance, String> {
    let material_files_result = deserialize::<MaterialInstanceFile>(data);
    match material_files_result {
        Err(_) => Err(String::from(
            "Could not deserialize the given material file.",
        )),
        Ok(material_instance_file) => {
            make_material_instance_from(asset_registry, &material_instance_file)
        }
    }
}

fn make_mesh_data_from(context: &WebGlRenderingContext, mesh_file: &MeshFile) -> MeshData {
    let (buffers, vertex_count) = deindex_buffers(
        context,
        mesh_file.buffers.as_slice(),
        mesh_file.triangles.as_slice(),
    );
    let mut mesh_data = MeshData::new(mesh_file.id.clone(), vertex_count);
    for buffer in buffers {
        mesh_data.push_buffer(buffer);
    }
    mesh_data
}

fn make_material_from(
    context: &WebGlRenderingContext,
    mat_file: &MaterialFile,
) -> Result<Material, String> {
    let material_result = Material::new(
        context,
        &mat_file.vertex_shader,
        &mat_file.framgent_shader,
        &mat_file.id,
    );
    match material_result {
        Ok(mut material) => {
            for uniform_data in &mat_file.global_uniforms {
                let value = make_uniform_value_from((uniform_data.1).0, &(uniform_data.1).1);
                let uniform = Uniform::new(uniform_data.0, value);
                material.set_uniform(uniform);
            }
            Ok(material)
        }
        Err(message) => Err(message),
    }
}

fn make_material_instance_from(
    asset_registry: &AssetRegistry,
    mat_instance_file: &MaterialInstanceFile,
) -> Result<MaterialInstance, String> {
    match asset_registry.get_material(&mat_instance_file.parent_id) {
        Some(mat) => {
            let mut mat_instance = MaterialInstance::new(mat, &mat_instance_file.id);
            for uniform_data in &mat_instance_file.uniforms {
                let value = make_uniform_value_from((uniform_data.1).0, &(uniform_data.1).1);
                let uniform = Uniform::new(uniform_data.0, value);
                mat_instance.set_uniform(uniform);
            }
            Ok(mat_instance)
        }
        None => Err(String::from(
            "Could not find parent material. Has it been registered yet?",
        )),
    }
}

fn make_uniform_value_from(value_type: ShaderDataType, fv: &FileValue) -> Box<dyn UniformValue> {
    match fv {
        FileValue::F32Array(fvec) => Box::new((value_type, fvec.clone())),
        FileValue::I16Array(ivec) => Box::new((value_type, ivec.clone())),
        FileValue::U8Array(uvec) => Box::new((value_type, uvec.clone())),
    }
}

fn deindex_buffers(
    context: &WebGlRenderingContext,
    buffers: &[FileBuffer],
    triangles: &[Triangle],
) -> (Vec<Buffer>, i32) {
    let vertex_buffer = get_buffer_with_name(buffers, "a_position");
    let normals_buffer = get_buffer_with_name(buffers, "a_normal");
    let uv_buffer = get_buffer_with_name(buffers, "a_tex_coordinates");
    let mut vertex_data = Vec::new();
    let mut normals_data = Vec::new();
    let mut uv_data = Vec::new();
    let mut result_vec = Vec::new();
    for triangle in triangles {
        deindex_triangle_in(Some(triangle.vertices), &vertex_buffer, &mut vertex_data);
        deindex_triangle_in(triangle.normals, &normals_buffer, &mut normals_data);
        deindex_triangle_in(triangle.uv, &uv_buffer, &mut uv_data);
    }
    let vertex_count = (vertex_data.len() / 3) as i32;
    if let Some(file_buffer) = vertex_buffer {
        let real_v_buffer_data = Buffer::from_f32_data_view(
            context,
            &file_buffer.name,
            ShaderDataType::Vector3,
            vertex_data.as_slice(),
        );
        result_vec.push(real_v_buffer_data);
    }
    if let Some(file_buffer) = normals_buffer {
        let real_n_buffer_data = Buffer::from_f32_data_view(
            context,
            &file_buffer.name,
            ShaderDataType::Vector3,
            normals_data.as_slice(),
        );
        result_vec.push(real_n_buffer_data);
    }
    if let Some(file_buffer) = uv_buffer {
        let real_u_buffer_data = Buffer::from_f32_data_view(
            context,
            &file_buffer.name,
            ShaderDataType::Vector2,
            uv_data.as_slice(),
        );
        result_vec.push(real_u_buffer_data);
    }
    (
        result_vec,
        vertex_count,
    )
}

fn deindex_triangle_in(
    data: Option<(u32, u32, u32)>,
    buffer: &Option<&FileBuffer>,
    data_vec: &mut Vec<f32>,
) -> () {
    if let Some((a, b, c)) = data {
        let triangle_iter = [a, b, c];
        if let Some(file_buffer) = buffer {
            let size = file_buffer.data_type.get_size() as u32;
            for i in triangle_iter.iter() {
                for j in (size * i)..(size * (i + 1)) {
                    if let FileValue::F32Array(f32_buffer) = &file_buffer.data {
                        data_vec.push(f32_buffer[j as usize]);
                    }
                }
            }
        }
    }
}

fn get_buffer_with_name<'a>(buffers: &'a [FileBuffer], name: &str) -> Option<&'a FileBuffer> {
    for buffer in buffers {
        if buffer.name == name {
            return Some(buffer);
        }
    }
    None
}
