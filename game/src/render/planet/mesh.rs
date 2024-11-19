use bevy::prelude::*;
use bevy::utils::HashMap;

pub trait VerticesExt {
    fn add_vertex(&mut self, unused_vertices: &mut UnusedVertices, position: [f32; 3]) -> u32;
    fn remove_vertex(
        &mut self,
        unused_vertices: &mut UnusedVertices,
        cache: &mut MidpointIndexCache,
        index: u32,
    );
    fn get_vertex(&self, index: u32) -> &[f32; 3];
    fn clear_all(&mut self);
}

pub trait IndicesExt {
    fn add_face(
        &mut self,
        unused_indices: &mut UnusedIndices,
        vertex_rc: &mut VertexRc,
        i1: u32,
        i2: u32,
        i3: u32,
    ) -> usize;
    fn remove_face(&mut self, unused_indices: &mut UnusedIndices, position: usize);
    fn replace_face(
        &mut self,
        vertex_rc: &mut VertexRc,
        position: usize,
        i1: u32,
        i2: u32,
        i3: u32,
    );
    fn get_face(&self, index: usize) -> (u32, u32, u32);
    fn clear_all(&mut self);

    fn remove_face_and_vertices(
        &mut self,
        cache: &mut MidpointIndexCache,
        unused_indices: &mut UnusedIndices,
        unused_vertices: &mut UnusedVertices,
        vertex_rc: &mut VertexRc,
        vertices: &mut impl VerticesExt,
        position: usize,
    ) {
        let (i1, i2, i3) = self.get_face(position);
        self.remove_face(unused_indices, position);

        if vertex_rc.decrement(i1) {
            vertices.remove_vertex(unused_vertices, cache, i1);
        }
        if vertex_rc.decrement(i2) {
            vertices.remove_vertex(unused_vertices, cache, i2);
        }
        if vertex_rc.decrement(i3) {
            vertices.remove_vertex(unused_vertices, cache, i3);
        }
    }

    fn get_midpoint(
        &mut self,
        cache: &mut MidpointIndexCache,
        vertices: &mut impl VerticesExt,
        unused_vertices: &mut UnusedVertices,
        i1: u32,
        i2: u32,
    ) -> u32 {
        if let Some(index) = cache.get(i1, i2) {
            return *index;
        }

        let p1 = vertices.get_vertex(i1);
        let p2 = vertices.get_vertex(i2);

        let mut midpoint = [0.0, 0.0, 0.0];
        for i in 0..3 {
            midpoint[i] = (p1[i] + p2[i]) / 2.0;
        }

        let midpoint_index = vertices.add_vertex(unused_vertices, midpoint);
        cache.insert(i1, i2, midpoint_index);

        midpoint_index
    }

    fn subdivide_face(
        &mut self,
        cache: &mut MidpointIndexCache,
        vertices: &mut impl VerticesExt,
        unused_indices: &mut UnusedIndices,
        unused_vertices: &mut UnusedVertices,
        vertex_rc: &mut VertexRc,
        index: usize,
        lod: usize,
    ) -> ([u32; 3], Vec<usize>) {
        let mut faces = vec![index];

        let mut second_level_indices = [0; 3];

        for current_lod in 0..lod {
            let mut new_faces = Vec::with_capacity(faces.len() * 4);
            for face_index in faces {
                let (i1, i2, i3) = self.get_face(face_index);

                let m1 = self.get_midpoint(cache, vertices, unused_vertices, i1, i2);
                let m2 = self.get_midpoint(cache, vertices, unused_vertices, i2, i3);
                let m3 = self.get_midpoint(cache, vertices, unused_vertices, i3, i1);

                // Removing the original face by replacing it with a new one
                self.replace_face(vertex_rc, face_index, i1, m1, m3);
                let f2 = self.add_face(unused_indices, vertex_rc, m1, i2, m2);
                let f3 = self.add_face(unused_indices, vertex_rc, i3, m3, m2);
                let f4 = self.add_face(unused_indices, vertex_rc, m1, m2, m3);

                // First partition (second level)
                if current_lod == 0 {
                    second_level_indices[0] = m1;
                    second_level_indices[1] = m2;
                    second_level_indices[2] = m3;
                }
                new_faces.extend(vec![face_index, f2, f3, f4]);
            }

            faces = new_faces;
        }
        (second_level_indices, faces)
    }
}

#[derive(Default, Component)]
pub struct MidpointIndexCache {
    map: HashMap<(u32, u32), u32>,
    keys: HashMap<u32, Vec<((u32, u32), (u32, u32))>>,
}

impl MidpointIndexCache {
    fn insert(&mut self, i1: u32, i2: u32, vertex_index: u32) {
        let key = if i1 < i2 { (i1, i2) } else { (i2, i1) };

        self.map.insert(key, vertex_index);

        self.keys
            .entry(i1)
            .or_insert_with(Vec::new)
            .push((key, (i2, vertex_index)));
        self.keys
            .entry(i2)
            .or_insert_with(Vec::new)
            .push((key, (i1, vertex_index)));
        self.keys
            .entry(vertex_index)
            .or_insert_with(Vec::new)
            .push((key, (i1, i2)));
    }

    fn get(&mut self, i1: u32, i2: u32) -> Option<&u32> {
        let key = if i1 < i2 { (i1, i2) } else { (i2, i1) };

        self.map.get(&key)
    }

    fn remove(&mut self, index: u32) {
        if let Some(values) = self.keys.remove(&index) {
            for value in values {
                let key = value.0;
                self.map.remove(&key);

                self.remove(value.1 .0);
                self.remove(value.1 .1);
            }
        }
    }

    pub fn clear_all(&mut self) {
        self.map.clear();
        self.keys.clear();
    }
}

#[derive(Default, Component)]
pub struct UnusedVertices(Vec<u32>);

impl UnusedVertices {
    fn add(&mut self, vertex: u32) {
        self.0.push(vertex);
    }

    fn get(&mut self) -> Option<u32> {
        self.0.pop()
    }

    pub fn clear_all(&mut self) {
        self.0.clear();
    }
}

#[derive(Default, Component)]
pub struct UnusedIndices(Vec<usize>);

impl UnusedIndices {
    fn add(&mut self, index: usize) {
        self.0.push(index);
    }

    fn get(&mut self) -> Option<usize> {
        self.0.pop()
    }

    pub fn clear_all(&mut self) {
        self.0.clear();
    }
}

#[derive(Default, Component)]
pub struct VertexRc(Vec<u8>);

impl VertexRc {
    fn increment(&mut self, index: u32) {
        if index >= self.0.len() as u32 {
            self.0.extend(vec![0; index as usize - self.0.len() + 1]);
        }

        self.0[index as usize] += 1;
    }

    /// Decrements count in that index, returns true if the new count is 0.
    /// Panics if the index is invalid
    fn decrement(&mut self, index: u32) -> bool {
        self.0[index as usize] -= 1;
        self.0[index as usize] == 0
    }

    pub fn clear_all(&mut self) {
        self.0.clear();
    }
}

impl VerticesExt for Vec<[f32; 3]> {
    fn add_vertex(&mut self, unused_vertices: &mut UnusedVertices, position: [f32; 3]) -> u32 {
        if let Some(index) = unused_vertices.get() {
            self[index as usize] = position;
            return index;
        }

        self.push(position);
        self.len() as u32 - 1
    }

    fn remove_vertex(
        &mut self,
        unused_vertices: &mut UnusedVertices,
        cache: &mut MidpointIndexCache,
        index: u32,
    ) {
        unused_vertices.add(index);
        cache.remove(index);
    }

    fn get_vertex(&self, index: u32) -> &[f32; 3] {
        self.get(index as usize)
            .expect("Vertex does not exist at that position")
    }

    fn clear_all(&mut self) {
        self.clear();
    }
}

impl IndicesExt for Vec<u32> {
    fn add_face(
        &mut self,
        unused_indices: &mut UnusedIndices,
        vertex_rc: &mut VertexRc,
        i1: u32,
        i2: u32,
        i3: u32,
    ) -> usize {
        vertex_rc.increment(i1);
        vertex_rc.increment(i2);
        vertex_rc.increment(i3);

        if let Some(index_position) = unused_indices.get() {
            self[index_position + 0] = i1;
            self[index_position + 1] = i2;
            self[index_position + 2] = i3;
            return index_position;
        }

        self.extend(vec![i1, i2, i3]);
        self.len() - 3
    }

    fn remove_face(&mut self, unused_indices: &mut UnusedIndices, position: usize) {
        unused_indices.add(position);

        self[position + 0] = 0;
        self[position + 1] = 0;
        self[position + 2] = 0;
    }

    fn replace_face(
        &mut self,
        vertex_rc: &mut VertexRc,
        position: usize,
        i1: u32,
        i2: u32,
        i3: u32,
    ) {
        vertex_rc.increment(i1);
        vertex_rc.increment(i2);
        vertex_rc.increment(i3);

        self[position + 0] = i1;
        self[position + 1] = i2;
        self[position + 2] = i3;
    }

    fn get_face(&self, index: usize) -> (u32, u32, u32) {
        let face = self.get(index..index + 3).expect("Face does not exist");

        (face[0], face[1], face[2])
    }

    fn clear_all(&mut self) {
        self.clear();
    }
}
