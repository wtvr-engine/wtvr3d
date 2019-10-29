use crate::component::mesh::MeshData;
use crate::renderer::buffer::Buffer;
use bincode::deserialize;
use web_sys::WebGlRenderingContext;
/// Deserializer for files generated from Collada using the wtvr3d Asset Converter
use wtvr3d_file::{FileBuffer, FileValue, MeshFile, ShaderDataType, Triangle};

pub fn deserialize_wmesh(
    context: &WebGlRenderingContext,
    data: &[u8],
) -> Result<Vec<MeshData>, String> {
    let mut result = Vec::new();
    let mesh_files_result = deserialize::<MeshFile>(data);
    match mesh_files_result {
        Err(_) => {
            return Err(String::from("Could not deserialize the given file :"));
        }
        Ok(mesh_file) => {
            result.push(make_mesh_data_from(context, &mesh_file));
        }
    }
    Ok(result)
}

fn make_mesh_data_from(context: &WebGlRenderingContext, mesh_file: &MeshFile) -> MeshData {
    let (buffers, vertex_count) = deindex_buffers(
        context,
        mesh_file.buffers.as_slice(),
        mesh_file.triangles.as_slice(),
    );
    let mut mesh_data = MeshData::new(vertex_count);
    for buffer in buffers {
        mesh_data.push_buffer(buffer);
    }
    mesh_data
}

fn deindex_buffers(
    context: &WebGlRenderingContext,
    buffers: &[FileBuffer],
    triangles: &[Triangle],
) -> (Vec<Buffer>, i32) {
    let vertex_buffer = get_buffer_with_name(buffers, "vertices");
    let normals_buffer = get_buffer_with_name(buffers, "normals");
    let uv_buffer = get_buffer_with_name(buffers, "tex_coordinates");
    let mut vertex_data = Vec::new();
    let mut normals_data = Vec::new();
    let mut uv_data = Vec::new();
    for triangle in triangles {
        deindex_triangle_in(Some(triangle.vertices), &vertex_buffer, &mut vertex_data);
        deindex_triangle_in(triangle.normals, &normals_buffer, &mut normals_data);
        deindex_triangle_in(triangle.uv, &uv_buffer, &mut uv_data);
    }
    let vertex_count = (vertex_data.len() / 3) as i32;
    let real_v_buffer_data = Buffer::from_f32_data_view(
        context,
        "vertices",
        ShaderDataType::Vector3,
        vertex_data.as_slice(),
    );
    let real_n_buffer_data = Buffer::from_f32_data_view(
        context,
        "normals",
        ShaderDataType::Vector3,
        normals_data.as_slice(),
    );
    let real_u_buffer_data = Buffer::from_f32_data_view(
        context,
        "tex_coordinates",
        ShaderDataType::Vector2,
        uv_data.as_slice(),
    );
    (
        vec![real_v_buffer_data, real_n_buffer_data, real_u_buffer_data],
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
