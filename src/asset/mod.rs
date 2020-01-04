//! Deserializer for files generated using the wtvr3d Asset Converter
mod asset_registry;

pub use asset_registry::AssetRegistry;

use crate::renderer::{Buffer, Material, MaterialInstance, MeshData, Uniform, UniformValue};
use bincode::deserialize;
use web_sys::WebGlRenderingContext;
use wtvr3d_file::{FileValue, MaterialFile, MaterialInstanceFile, MeshFile, ShaderDataType};

pub fn deserialize_wmesh(context: &WebGlRenderingContext, data: &[u8]) -> Result<MeshData, String> {
    let mesh_files_result = deserialize::<MeshFile>(data);
    match mesh_files_result {
        Err(_) => Err(String::from("Could not deserialize the given mesh file.")),
        Ok(mesh_file) => Ok(make_mesh_data_from(context, &mesh_file)),
    }
}

pub fn deserialize_wmaterial(
    asset_registry: &AssetRegistry,
    data: &[u8],
) -> Result<Material, String> {
    let material_files_result = deserialize::<MaterialFile>(data);
    match material_files_result {
        Err(_) => Err(String::from(
            "Could not deserialize the given material file.",
        )),
        Ok(material_file) => Ok(make_material_from(asset_registry, &material_file)),
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

// ⭕ TODO : handle other FileValue types if anything else is provided
fn make_mesh_data_from(context: &WebGlRenderingContext, mesh_file: &MeshFile) -> MeshData {
    let mut v_indexes = Vec::new();
    for triangle in &mesh_file.triangles {
        v_indexes.push(triangle.vertices.0 as u16);
        v_indexes.push(triangle.vertices.1 as u16);
        v_indexes.push(triangle.vertices.2 as u16);
    }
    let mut mesh_data = MeshData::new(mesh_file.id.clone(), mesh_file.triangles.len() as i32 * 3);
    for buffer in &mesh_file.buffers {
        if let FileValue::F32Array(buffer_data) = &buffer.data {
            let indexes = match buffer.name.as_str() {
                crate::utils::constants::VERTEX_BUFFER_NAME => Some(v_indexes.as_slice()),
                _ => None,
            };
            let buf = Buffer::from_f32_data_view(context, &buffer.name, buffer.data_type, buffer_data,indexes);
            mesh_data.push_buffer(buf);
        }
    }
    mesh_data
}

fn make_material_from(asset_registry: &AssetRegistry, mat_file: &MaterialFile) -> Material {
    let mut material = Material::new(
        &mat_file.vertex_shader,
        &mat_file.framgent_shader,
        &mat_file.id,
    );
    let mut max_texture = 0;
    for uniform_data in &mat_file.global_uniforms {
        let value =
            make_uniform_value_from((uniform_data.1).0, &(uniform_data.1).1, asset_registry)
                .unwrap();
        let mut uniform = Uniform::new(uniform_data.0, value);
        if (uniform_data.1).0 == ShaderDataType::Sampler2D {
            uniform.set_texture_index(max_texture);
            max_texture += 1;
        }
        material.set_uniform(uniform);
    }
    material
}

fn make_material_instance_from(
    asset_registry: &AssetRegistry,
    mat_instance_file: &MaterialInstanceFile,
) -> Result<MaterialInstance, String> {
    match asset_registry.get_material(&mat_instance_file.parent_id) {
        Some(mat) => {
            let mut mat_instance = MaterialInstance::new(mat.clone(), &mat_instance_file.id);
            let parent_texture_indexes = &mat.borrow().get_texture_indexes().unwrap();
            let mut next_index = 0;
            for (_, index) in parent_texture_indexes {
                if index >= &next_index {
                    next_index = index + 1;
                }
            }
            for uniform_data in &mat_instance_file.uniforms {
                let value = make_uniform_value_from(
                    (uniform_data.1).0,
                    &(uniform_data.1).1,
                    asset_registry,
                )
                .unwrap();
                let mut uniform = Uniform::new(uniform_data.0, value);
                if (uniform_data.1).0 == ShaderDataType::Sampler2D {
                    if parent_texture_indexes.contains_key(uniform_data.0) {
                        uniform.set_texture_index(
                            parent_texture_indexes.get(uniform_data.0).unwrap().clone(),
                        );
                    } else {
                        uniform.set_texture_index(next_index);
                        next_index += 1;
                    }
                }
                mat_instance.set_uniform(uniform);
            }
            Ok(mat_instance)
        }
        None => Err(String::from(
            "Could not find parent material. Has it been registered yet?",
        )),
    }
}

fn make_uniform_value_from(
    value_type: ShaderDataType,
    fv: &FileValue,
    asset_registry: &AssetRegistry,
) -> Result<Box<dyn UniformValue>, String> {
    match fv {
        FileValue::F32Array(fvec) => Ok(Box::new((value_type, fvec.clone()))),
        FileValue::I16Array(ivec) => Ok(Box::new((value_type, ivec.clone()))),
        FileValue::U8Array(uvec) => Ok(Box::new((value_type, uvec.clone()))),
        FileValue::AssetID(id) => match asset_registry.get_texture(&id) {
            Some(rc) => Ok(Box::new(rc)),
            None => Err(format!(
                "Texture with id {} does not exist. Has it been registered yet?",
                id
            )),
        },
        _ => Err(String::from("Unknown FileValue reached.")),
    }
}
