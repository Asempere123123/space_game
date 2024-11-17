use std::rc::Rc;

use super::mesh::{IndicesExt, VerticesExt};

const CHUNK_LOD: usize = 5;

struct Chunk {
    children: Vec<Chunk>,
    parent: Option<Rc<Chunk>>,

    lod: u8,
    position: (f32, f32, f32),

    vertices: Vec<u32>,
    faces: Vec<u32>,
}

impl Chunk {
    fn new(lod: u8, position: (f32, f32, f32), parent: Option<Rc<Chunk>>) -> Self {
        Self {
            children: Vec::new(),
            parent: parent,

            lod,
            position,

            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    fn subdivide(
        &mut self,
        mesh_indices: &mut impl IndicesExt,
        mesh_vertices: &mut impl VerticesExt,
    ) {
        //mesh_indices.subdivide_face(cache, vertices, index, CHUNK_LOD);
    }
}
