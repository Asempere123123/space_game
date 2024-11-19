use std::{rc::Rc, sync::RwLock};

use super::mesh::{
    IndicesExt, MidpointIndexCache, UnusedIndices, UnusedVertices, VertexRc, VerticesExt,
};

const CHUNK_LOD: usize = 5;

pub struct Chunk {
    pub children: Vec<Chunk>,
    is_root: bool,

    position: (f32, f32, f32),

    first_level_vertices: [u32; 3],
    second_level_indices: [u32; 3],
    faces: Vec<usize>,
}

impl Chunk {
    pub fn new(
        position: (f32, f32, f32),
        is_root: bool,

        first_level_vertices: [u32; 3],

        mesh_indices: &mut impl IndicesExt,
        mesh_vertices: &mut impl VerticesExt,
        unused_indices: &mut UnusedIndices,
        unused_vertices: &mut UnusedVertices,
        vertex_rc: &mut VertexRc,
        cache: &mut MidpointIndexCache,
    ) -> Self {
        // Do the divisions of the chunk
        let root_face_index = mesh_indices.add_face(
            unused_indices,
            vertex_rc,
            first_level_vertices[0],
            first_level_vertices[1],
            first_level_vertices[2],
        );
        let (second_level_indices, faces) = mesh_indices.subdivide_face(
            cache,
            mesh_vertices,
            unused_indices,
            unused_vertices,
            vertex_rc,
            root_face_index,
            CHUNK_LOD,
        );

        Self {
            children: Vec::new(),
            is_root,

            position,

            first_level_vertices,
            second_level_indices,
            faces,
        }
    }

    pub fn subdivide(
        &mut self,
        mesh_indices: &mut impl IndicesExt,
        mesh_vertices: &mut impl VerticesExt,
        unused_indices: &mut UnusedIndices,
        unused_vertices: &mut UnusedVertices,
        vertex_rc: &mut VertexRc,
        cache: &mut MidpointIndexCache,
    ) {
        debug_assert!(self.children.is_empty());

        for face in &self.faces {
            mesh_indices.remove_face_and_vertices(
                cache,
                unused_indices,
                unused_vertices,
                vertex_rc,
                mesh_vertices,
                *face,
            );
        }

        self.children.reserve(4);
        self.children.push(Chunk::new(
            (0.0, 0.0, 0.0),
            false,
            [
                self.first_level_vertices[0],
                self.second_level_indices[0],
                self.second_level_indices[2],
            ],
            mesh_indices,
            mesh_vertices,
            unused_indices,
            unused_vertices,
            vertex_rc,
            cache,
        ));
        self.children.push(Chunk::new(
            (0.0, 0.0, 0.0),
            false,
            [
                self.second_level_indices[0],
                self.second_level_indices[1],
                self.second_level_indices[2],
            ],
            mesh_indices,
            mesh_vertices,
            unused_indices,
            unused_vertices,
            vertex_rc,
            cache,
        ));
        self.children.push(Chunk::new(
            (0.0, 0.0, 0.0),
            false,
            [
                self.second_level_indices[0],
                self.first_level_vertices[1],
                self.second_level_indices[1],
            ],
            mesh_indices,
            mesh_vertices,
            unused_indices,
            unused_vertices,
            vertex_rc,
            cache,
        ));
        self.children.push(Chunk::new(
            (0.0, 0.0, 0.0),
            false,
            [
                self.second_level_indices[1],
                self.first_level_vertices[2],
                self.second_level_indices[2],
            ],
            mesh_indices,
            mesh_vertices,
            unused_indices,
            unused_vertices,
            vertex_rc,
            cache,
        ));
    }

    fn clear(
        &mut self,
        mesh_indices: &mut impl IndicesExt,
        mesh_vertices: &mut impl VerticesExt,
        unused_indices: &mut UnusedIndices,
        unused_vertices: &mut UnusedVertices,
        vertex_rc: &mut VertexRc,
        cache: &mut MidpointIndexCache,
    ) {
        debug_assert!(self.children.is_empty());

        for face in &self.faces {
            mesh_indices.remove_face_and_vertices(
                cache,
                unused_indices,
                unused_vertices,
                vertex_rc,
                mesh_vertices,
                *face,
            );
        }
    }

    fn clear_all(
        &mut self,
        mesh_indices: &mut impl IndicesExt,
        mesh_vertices: &mut impl VerticesExt,
        unused_indices: &mut UnusedIndices,
        unused_vertices: &mut UnusedVertices,
        vertex_rc: &mut VertexRc,
        cache: &mut MidpointIndexCache,
    ) {
        debug_assert!(self.is_root);

        self.children = Vec::new();
        let first_first_level_vertex_pos = *mesh_vertices.get_vertex(self.first_level_vertices[0]);
        let second_first_level_vertex_pos = *mesh_vertices.get_vertex(self.first_level_vertices[1]);
        let third_first_level_vertex_pos = *mesh_vertices.get_vertex(self.first_level_vertices[2]);

        // Clear all
        mesh_indices.clear_all();
        mesh_vertices.clear_all();
        unused_indices.clear_all();
        unused_vertices.clear_all();
        vertex_rc.clear_all();
        cache.clear_all();

        self.first_level_vertices[0] =
            mesh_vertices.add_vertex(unused_vertices, first_first_level_vertex_pos);
        self.first_level_vertices[1] =
            mesh_vertices.add_vertex(unused_vertices, second_first_level_vertex_pos);
        self.first_level_vertices[2] =
            mesh_vertices.add_vertex(unused_vertices, third_first_level_vertex_pos);

        // Do the divisions of the chunk
        let root_face_index = mesh_indices.add_face(
            unused_indices,
            vertex_rc,
            self.first_level_vertices[0],
            self.first_level_vertices[1],
            self.first_level_vertices[2],
        );
        let (second_level_indices, faces) = mesh_indices.subdivide_face(
            cache,
            mesh_vertices,
            unused_indices,
            unused_vertices,
            vertex_rc,
            root_face_index,
            CHUNK_LOD,
        );
        self.second_level_indices = second_level_indices;
        self.faces = faces;
    }

    pub fn undivide(
        &mut self,
        mesh_indices: &mut impl IndicesExt,
        mesh_vertices: &mut impl VerticesExt,
        unused_indices: &mut UnusedIndices,
        unused_vertices: &mut UnusedVertices,
        vertex_rc: &mut VertexRc,
        cache: &mut MidpointIndexCache,
    ) {
        debug_assert!(!self.children.is_empty());

        if self.is_root {
            self.clear_all(
                mesh_indices,
                mesh_vertices,
                unused_indices,
                unused_vertices,
                vertex_rc,
                cache,
            );
            return;
        }

        // This leaks vertices (i think)
        while let Some(mut child) = self.children.pop() {
            child.clear(
                mesh_indices,
                mesh_vertices,
                unused_indices,
                unused_vertices,
                vertex_rc,
                cache,
            );
        }

        // Do the divisions of the chunk
        let root_face_index = mesh_indices.add_face(
            unused_indices,
            vertex_rc,
            self.first_level_vertices[0],
            self.first_level_vertices[1],
            self.first_level_vertices[2],
        );
        let (second_level_indices, faces) = mesh_indices.subdivide_face(
            cache,
            mesh_vertices,
            unused_indices,
            unused_vertices,
            vertex_rc,
            root_face_index,
            CHUNK_LOD,
        );

        self.second_level_indices = second_level_indices;
        self.faces = faces;
    }
}
