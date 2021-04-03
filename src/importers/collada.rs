//! Collada importer module for Mesh

use crate::{
    asset::{Buffer, Mesh},
    error::W3DError,
};

use collada::{document::ColladaDocument, ObjSet};
use collada::{Object, PrimitiveElement, TVertex, Triangles, Vertex};
use nalgebra::{Vector2, Vector3};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct ColladaTriangle {
    pub vertices: (usize, usize, usize),
    pub normals: Option<(usize, usize, usize)>,
    pub uvs: Option<(usize, usize, usize)>,
    pub tangeant: Option<Vector3<f32>>,
}

impl ColladaTriangle {
    pub fn get_normal_for_vertex(&self, index: usize) -> Option<usize> {
        if index == self.vertices.0 {
            self.normals.map(|trio| trio.0)
        } else if index == self.vertices.1 {
            self.normals.map(|trio| trio.1)
        } else if index == self.vertices.2 {
            self.normals.map(|trio| trio.2)
        } else {
            None
        }
    }

    pub fn get_uv_for_vertex(&self, index: usize) -> Option<usize> {
        if index == self.vertices.0 {
            self.uvs.map(|trio| trio.0)
        } else if index == self.vertices.1 {
            self.uvs.map(|trio| trio.1)
        } else if index == self.vertices.2 {
            self.uvs.map(|trio| trio.2)
        } else {
            None
        }
    }

    pub fn replace_vertex(&mut self, old_index: usize, new_index: usize) {
        if old_index == self.vertices.0 {
            self.vertices.0 = new_index;
        } else if old_index == self.vertices.1 {
            self.vertices.1 = new_index;
        } else if old_index == self.vertices.2 {
            self.vertices.2 = new_index;
        }
    }

    pub fn replace_uv(&mut self, old_index: usize, new_index: usize) {
        if let Some(mut uvs) = self.uvs {
            if old_index == uvs.0 {
                uvs.0 = new_index;
            } else if old_index == uvs.1 {
                uvs.1 = new_index;
            } else if old_index == uvs.2 {
                uvs.2 = new_index;
            }
            self.uvs = Some(uvs);
        }
    }

    pub fn replace_normal(&mut self, old_index: usize, new_index: usize) {
        if let Some(mut normals) = self.normals {
            if old_index == normals.0 {
                normals.0 = new_index;
            } else if old_index == normals.1 {
                normals.1 = new_index;
            } else if old_index == normals.2 {
                normals.2 = new_index;
            }
            self.normals = Some(normals);
        }
    }
}

#[derive(Default, Clone)]
struct RawColladaData {
    pub vertices: Vec<f32>,
    pub normals: Option<Vec<f32>>,
    pub uvs: Option<Vec<f32>>,
    pub joint_weights: Option<Vec<f32>>,
    pub tangeants: Option<Vec<f32>>,
    pub mono_index: bool,
}

impl RawColladaData {
    pub fn new(
        vertices: &[f32],
        normals: Option<&[f32]>,
        uvs: Option<&[f32]>,
        joint_weights: Option<&[f32]>,
    ) -> RawColladaData {
        RawColladaData {
            vertices: vertices.to_vec(),
            normals: normals.map(|value| value.to_vec()),
            uvs: uvs.map(|value| value.to_vec()),
            joint_weights: joint_weights.map(|value| value.to_vec()),
            tangeants: None,
            mono_index: false,
        }
    }

    pub fn new_mono_from_multi_indexed(
        original: &RawColladaData,
        triangles: &Vec<Rc<RefCell<ColladaTriangle>>>,
    ) -> RawColladaData {
        let [normals_hash, uvs_hash] = RawColladaData::gather_indexes(triangles);
        RawColladaData {
            vertices: original.vertices.clone(),
            normals: original
                .normals
                .clone()
                .map(|vec| RawColladaData::reindex_objects(vec, &normals_hash, 3)),
            uvs: original
                .uvs
                .clone()
                .map(|vec| RawColladaData::reindex_objects(vec, &uvs_hash, 2)),
            joint_weights: original.joint_weights.clone(),
            tangeants: original.tangeants.clone(),
            mono_index: true,
        }
    }

    pub fn duplicate_vertex(&mut self, index: usize) -> usize {
        let new_index = self.vertices.len() / 3;
        self.vertices.push(self.vertices[index * 3]);
        self.vertices.push(self.vertices[index * 3 + 1]);
        self.vertices.push(self.vertices[index * 3 + 2]);
        if let Some(tangeants) = &mut self.tangeants {
            tangeants.push(tangeants[index * 3]);
            tangeants.push(tangeants[index * 3 + 1]);
            tangeants.push(tangeants[index * 3 + 2]);
        }

        if let Some(joint_weights) = &mut self.joint_weights {
            joint_weights.push(joint_weights[index * 4]);
            joint_weights.push(joint_weights[index * 4 + 1]);
            joint_weights.push(joint_weights[index * 4 + 2]);
            joint_weights.push(joint_weights[index * 4 + 3]);
        }
        new_index
    }

    pub fn uvs_are_equal(&self, index1: usize, index2: usize) -> bool {
        if let Some(uvs) = &self.uvs {
            uvs[index1 * 2] == uvs[index2 * 2] && uvs[index1 * 2 + 1] == uvs[index2 * 2 + 1]
        } else {
            false
        }
    }

    pub fn normals_are_equal(&self, index1: usize, index2: usize) -> bool {
        if let Some(normals) = &self.normals {
            normals[index1 * 3] == normals[index2 * 3]
                && normals[index1 * 3 + 1] == normals[index2 * 3 + 1]
                && normals[index1 * 3 + 2] == normals[index2 * 3 + 2]
        } else {
            false
        }
    }

    pub fn get_vertex_at(&self, i: usize) -> Vector3<f32> {
        Vector3::new(
            self.vertices[i * 3],
            self.vertices[i * 3 + 1],
            self.vertices[i * 3 + 2],
        )
    }

    pub fn get_uv_at(&self, i: usize) -> Option<Vector2<f32>> {
        if let Some(uvs) = &self.uvs {
            Some(Vector2::new(uvs[i * 2], uvs[i * 2 + 1]))
        } else {
            None
        }
    }

    fn reindex_objects(
        objects: Vec<f32>,
        indices: &HashMap<usize, usize>,
        object_size: usize,
    ) -> Vec<f32> {
        let mut result = zeros(object_size * indices.len());
        for (i, j) in indices {
            for k in 0..object_size {
                result[object_size * i + k] = objects[object_size * j + k];
            }
        }
        result
    }

    fn gather_indexes(triangles: &Vec<Rc<RefCell<ColladaTriangle>>>) -> [HashMap<usize, usize>; 2] {
        let mut result: [HashMap<usize, usize>; 2] = Default::default();
        for triangle_rc in triangles {
            let triangle = triangle_rc.borrow();
            if let Some(normal_indexes) = &triangle.normals {
                result[0].insert(triangle.vertices.0, normal_indexes.0);
                result[0].insert(triangle.vertices.1, normal_indexes.1);
                result[0].insert(triangle.vertices.2, normal_indexes.2);
            }
            if let Some(uv_indexes) = &triangle.uvs {
                result[1].insert(triangle.vertices.0, uv_indexes.0);
                result[1].insert(triangle.vertices.1, uv_indexes.1);
                result[1].insert(triangle.vertices.2, uv_indexes.2);
            }
        }
        result
    }
}

#[derive(Default)]
struct ColladaMesh {
    pub triangles: Vec<Rc<RefCell<ColladaTriangle>>>,
    pub data: RawColladaData,
}

impl ColladaMesh {
    pub fn new(object: Object) -> ColladaMesh {
        let mut triangles = Vec::new();
        for geometry in object.geometry {
            for shape in geometry.mesh {
                match shape {
                    PrimitiveElement::Triangles(tris) => {
                        triangles.append(&mut ColladaMesh::convert_triangles(tris));
                    }
                    _ => {}
                }
            }
        }
        let data = RawColladaData::new(
            &ColladaMesh::convert_vertices_to_f32(object.vertices),
            Some(&ColladaMesh::convert_vertices_to_f32(object.normals)),
            Some(&ColladaMesh::convert_tex_vertices_to_f32(
                object.tex_vertices,
            )),
            None,
        );
        ColladaMesh { triangles, data }
    }

    pub fn to_mesh(&self, name: &str) -> Mesh {
        let reindexed_data =
            RawColladaData::new_mono_from_multi_indexed(&self.data, &self.triangles);
        let vertex_buffer =
            Buffer::new_from_f32_data("a_position".to_string(), reindexed_data.vertices, 3);
        let normals_buffer = reindexed_data
            .normals
            .map(|normals| Buffer::new_from_f32_data("a_normal".to_string(), normals, 3));
        let uv_buffer = reindexed_data
            .uvs
            .map(|uvs| Buffer::new_from_f32_data("a_tex_coordinates".to_string(), uvs, 2));
        let tangeants_buffer = reindexed_data
            .tangeants
            .map(|tangeants| Buffer::new_from_f32_data("a_tangeant".to_string(), tangeants, 3));
        let mut indexes = Vec::new();
        for triangle in &self.triangles {
            let tri = triangle.borrow();
            indexes.push(tri.vertices.0 as u32);
            indexes.push(tri.vertices.1 as u32);
            indexes.push(tri.vertices.2 as u32);
        }
        let indexes_buffer = Buffer::new_from_u32_data(String::new(), indexes, 3);

        Mesh::new(
            name.to_string(),
            vertex_buffer,
            Some(indexes_buffer),
            normals_buffer,
            None,
            uv_buffer,
            tangeants_buffer,
        )
    }

    fn convert_triangles(triangles: Triangles) -> Vec<Rc<RefCell<ColladaTriangle>>> {
        let mut result = Vec::new();
        for i in 0..triangles.vertices.len() {
            let triangle_vertices = triangles.vertices[i].clone();
            let triangle_normals = if let Some(normals) = &triangles.normals {
                Some(normals[i].clone())
            } else {
                None
            };
            let triangle_tex = if let Some(tex) = &triangles.tex_vertices {
                Some(tex[i]).clone()
            } else {
                None
            };
            let triangle_data = ColladaTriangle {
                vertices: triangle_vertices,
                normals: triangle_normals,
                uvs: triangle_tex,
                tangeant: None,
            };
            result.push(Rc::new(RefCell::new(triangle_data)));
        }
        result
    }

    fn convert_vertices_to_f32(vertices: Vec<Vertex>) -> Vec<f32> {
        let mut result = Vec::new();
        for vertex in vertices {
            result.push(vertex.x as f32);
            result.push(vertex.y as f32);
            result.push(vertex.z as f32);
        }
        result
    }

    fn convert_tex_vertices_to_f32(vertices: Vec<TVertex>) -> Vec<f32> {
        let mut result = Vec::new();
        for vertex in vertices {
            result.push(vertex.x as f32);
            result.push(vertex.y as f32);
        }
        result
    }

    fn construct_tangeants(&mut self) {
        if self.data.uvs == None {
            return;
        }
        let mut tangeants_buffer = zeros(self.data.vertices.len());
        let index_map = self.triangle_indexes_by_vertex_index();
        for (vert, triangles) in index_map {
            let mut tangeant_average = Vector3::new(0.0, 0.0, 0.0);
            for triangle in &triangles {
                self.compute_tangeant(triangle.clone());
                tangeant_average += triangle.borrow().tangeant.unwrap();
            }
            tangeant_average = tangeant_average / triangles.len() as f32;
            tangeants_buffer[vert * 3] = tangeant_average.x;
            tangeants_buffer[vert * 3 + 1] = tangeant_average.y;
            tangeants_buffer[vert * 3 + 2] = tangeant_average.z;
        }
        self.data.tangeants = Some(tangeants_buffer);
    }

    fn compute_tangeant(&self, triangle: Rc<RefCell<ColladaTriangle>>) {
        let mut tri = triangle.borrow_mut();
        if tri.tangeant != None || tri.uvs == None {
            return;
        }
        let p1 = self.data.get_vertex_at(tri.vertices.0);
        let p2 = self.data.get_vertex_at(tri.vertices.1);
        let p3 = self.data.get_vertex_at(tri.vertices.2);
        let u1 = self.data.get_uv_at(tri.uvs.unwrap().0).unwrap();
        let u2 = self.data.get_uv_at(tri.uvs.unwrap().1).unwrap();
        let u3 = self.data.get_uv_at(tri.uvs.unwrap().2).unwrap();

        let delta_pos_1 = p2 - p1;
        let delta_pos_2 = p3 - p1;

        let delta_uv_1 = u2 - u1;
        let delta_uv_2 = u3 - u1;

        let r = 1.0 / (delta_uv_1.x * delta_uv_2.y - delta_uv_1.y * delta_uv_2.x);
        let tangeant = (delta_pos_1 * delta_uv_2.y - delta_pos_2 * delta_uv_1.y) * r;
        tri.tangeant = Some(tangeant);
    }

    fn simplify_indexes(&mut self) {
        let index_map = self.triangle_indexes_by_vertex_index();
        for (vert, triangles) in index_map {
            let mut vertex_normal_opt = None;
            for triangle_rc in triangles {
                let mut triangle = triangle_rc.borrow_mut();
                if let Some(normal) = triangle.get_normal_for_vertex(vert) {
                    if let Some(vertex_normal) = vertex_normal_opt {
                        if vertex_normal != normal {
                            if !self.data.normals_are_equal(normal, vertex_normal) {
                                triangle.replace_vertex(vert, self.data.duplicate_vertex(vert));
                            } else {
                                triangle.replace_normal(normal, vertex_normal);
                            }
                        }
                    } else {
                        vertex_normal_opt = Some(normal);
                    }
                }
            }
        }
        let index_map_2 = self.triangle_indexes_by_vertex_index();
        for (vert, triangles) in index_map_2 {
            let mut vertex_uv_opt = None;
            for triangle_rc in triangles {
                let mut triangle = triangle_rc.borrow_mut();
                if let Some(uv) = triangle.get_uv_for_vertex(vert) {
                    if let Some(vertex_uv) = vertex_uv_opt {
                        if vertex_uv != uv {
                            if !self.data.uvs_are_equal(uv, vertex_uv) {
                                triangle.replace_vertex(vert, self.data.duplicate_vertex(vert));
                            } else {
                                triangle.replace_uv(uv, vertex_uv);
                            }
                        }
                    } else {
                        vertex_uv_opt = Some(uv);
                    }
                }
            }
        }
    }

    fn triangle_indexes_by_vertex_index(
        &self,
    ) -> HashMap<usize, Vec<Rc<RefCell<ColladaTriangle>>>> {
        let mut result: HashMap<usize, Vec<Rc<RefCell<ColladaTriangle>>>> = Default::default();
        for triangle_rc in &self.triangles {
            let triangle = triangle_rc.borrow();
            ColladaMesh::insert_in_hashmap_vec(
                triangle.vertices.0,
                triangle_rc.clone(),
                &mut result,
            );
            ColladaMesh::insert_in_hashmap_vec(
                triangle.vertices.1,
                triangle_rc.clone(),
                &mut result,
            );
            ColladaMesh::insert_in_hashmap_vec(
                triangle.vertices.2,
                triangle_rc.clone(),
                &mut result,
            );
        }
        result
    }

    fn insert_in_hashmap_vec(
        key: usize,
        value: Rc<RefCell<ColladaTriangle>>,
        map: &mut HashMap<usize, Vec<Rc<RefCell<ColladaTriangle>>>>,
    ) {
        if map.contains_key(&key) {
            map.get_mut(&key).unwrap().push(value);
        } else {
            map.insert(key, vec![value]);
        }
    }
}

fn zeros(size: usize) -> Vec<f32> {
    let mut zero_vec: Vec<f32> = Vec::with_capacity(size);
    for _ in 0..size {
        zero_vec.push(0.0);
    }
    return zero_vec;
}

impl Mesh {
    pub fn from_collada(dae_file: String, name: &str) -> Result<Vec<Mesh>, W3DError> {
        let obj_set = Mesh::get_obj_set_from_dae(dae_file, name)?;
        let mut meshes = Vec::new();
        let multiple = obj_set.objects.len() > 1;
        let mut index: usize = 1;
        for obj in obj_set.objects {
            let mut mesh_name = name.to_string();
            if multiple {
                mesh_name.push_str(&index.to_string());
            }
            meshes.push(ColladaMesh::new(obj).to_mesh(&mesh_name));
            index = index + 1;
        }
        Ok(meshes)
    }

    fn get_obj_set_from_dae(dae_file: String, name: &str) -> Result<ObjSet, W3DError> {
        let document = ColladaDocument::from_str(dae_file.as_str()).map_err(|e| {
            W3DError::new_with_desc(
                "Could not parse DAE file",
                Some(name.to_string()),
                Some(e.to_string()),
            )
        })?;
        document.get_obj_set().ok_or_else(|| {
            W3DError::new(
                "No Object Set found in Collada document",
                Some(name.to_string()),
            )
        })
    }
}
