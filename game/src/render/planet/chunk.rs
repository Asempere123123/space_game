use bevy::prelude::*;

use super::mesh::{
    IndicesExt, MidpointIndexCache, UnusedIndices, UnusedVertices, VertexRc, VerticesExt,
};

const CHUNK_LOD: usize = 2;
#[cfg(not(target_arch = "wasm32"))]
const MAX_LOD: i32 = 22;
#[cfg(target_arch = "wasm32")]
const MAX_LOD: i32 = 16;
const SUBDIVIDE_RADIUS: f32 = 25.0;
#[cfg(not(target_arch = "wasm32"))]
const UNDIVIDE_RADIUS: f32 = 40.0;
#[cfg(target_arch = "wasm32")]
const UNDIVIDE_RADIUS: f32 = 30.0;

#[derive(Component)]
pub struct Chunk {
    pub children: Vec<Chunk>,
    is_root: bool,
    radius: f32,

    position: [f32; 3],
    lod: i32,

    first_level_vertices: [u32; 3],
    second_level_indices: [u32; 3],
    faces: Vec<usize>,
}

impl Chunk {
    pub fn new(
        is_root: bool,
        lod: i32,
        radius: f32,

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

        let v1 = mesh_vertices.get_vertex(first_level_vertices[0]);
        let v2 = mesh_vertices.get_vertex(first_level_vertices[1]);
        let v3 = mesh_vertices.get_vertex(first_level_vertices[2]);

        // The position will be the middle of the 3 vertices
        let position_x = (v1[0] + v2[0] + v3[0]) / 3.0;
        let position_y = (v1[1] + v2[1] + v3[1]) / 3.0;
        let position_z = (v1[2] + v2[2] + v3[2]) / 3.0;

        // Radius / norm
        let position_factor =
            radius / ((position_x.powi(2) + position_y.powi(2) + position_z.powi(2)).sqrt());
        let position = [
            position_x * position_factor,
            position_y * position_factor,
            position_z * position_factor,
        ];

        Self {
            children: Vec::new(),
            is_root,
            lod,
            radius,

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
        if self.lod >= MAX_LOD {
            return;
        }

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
            false,
            self.lod + 1,
            self.radius,
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
            false,
            self.lod + 1,
            self.radius,
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
            false,
            self.lod + 1,
            self.radius,
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
            false,
            self.lod + 1,
            self.radius,
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

    pub fn divide_or_undivide(
        &mut self,
        mesh_indices: &mut impl IndicesExt,
        mesh_vertices: &mut impl VerticesExt,
        unused_indices: &mut UnusedIndices,
        unused_vertices: &mut UnusedVertices,
        vertex_rc: &mut VertexRc,
        cache: &mut MidpointIndexCache,
        camera_position: &[f32; 3],
    ) {
        let mut stack = vec![self as *mut Chunk];
        let mut visited = Vec::new();

        while let Some(current_chunk) = stack.pop() {
            for child in &mut (unsafe { &mut *current_chunk }).children {
                stack.push(child as *mut Chunk);
            }

            visited.push(current_chunk);
        }

        while let Some(chunk) = visited.pop() {
            let chunk = unsafe { &mut *chunk };

            let distance_squared = (camera_position[0] - chunk.position[0]).powi(2)
                + (camera_position[1] - chunk.position[1]).powi(2)
                + (camera_position[2] - chunk.position[2]).powi(2);
            // To far, undivide
            if distance_squared
                > ((UNDIVIDE_RADIUS * self.radius) / 2.0_f32.powi(chunk.lod)).powi(2)
                && !chunk.children.is_empty()
            {
                chunk.undivide(
                    mesh_indices,
                    mesh_vertices,
                    unused_indices,
                    unused_vertices,
                    vertex_rc,
                    cache,
                );
            } else if distance_squared
                < ((SUBDIVIDE_RADIUS * self.radius) / 2.0_f32.powi(chunk.lod)).powi(2)
                && chunk.children.is_empty()
            {
                chunk.subdivide(
                    mesh_indices,
                    mesh_vertices,
                    unused_indices,
                    unused_vertices,
                    vertex_rc,
                    cache,
                );
            }
        }
    }
}
