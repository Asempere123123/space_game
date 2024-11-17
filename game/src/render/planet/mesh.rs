use bevy::utils::HashMap;

pub trait VerticesExt {
    fn add_vertex(&mut self, position: [f32; 3]) -> u32;
    fn get_vertex(&self, index: u32) -> &[f32; 3];
}

pub trait IndicesExt {
    fn add_face(&mut self, i1: u32, i2: u32, i3: u32) -> usize;
    fn replace_face(&mut self, position: usize, i1: u32, i2: u32, i3: u32);
    fn get_face(&self, index: usize) -> (u32, u32, u32);

    fn get_midpoint(
        &mut self,
        cache: &mut MidpointIndexCache,
        vertices: &mut impl VerticesExt,
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

        let midpoint_index = vertices.add_vertex(midpoint);
        cache.insert(i1, i2, midpoint_index);

        midpoint_index
    }

    fn subdivide_face(
        &mut self,
        cache: &mut MidpointIndexCache,
        vertices: &mut impl VerticesExt,
        index: usize,
        lod: usize,
    ) {
        let mut faces = vec![index];

        for _ in 0..lod {
            let mut new_faces = Vec::with_capacity(faces.len() * 4);
            for face_index in faces {
                let (i1, i2, i3) = self.get_face(face_index);

                let m1 = self.get_midpoint(cache, vertices, i1, i2);
                let m2 = self.get_midpoint(cache, vertices, i2, i3);
                let m3 = self.get_midpoint(cache, vertices, i3, i1);

                // Removing the original face by replacing it with a new one
                self.replace_face(face_index, i1, m1, m3);
                let f2 = self.add_face(m1, i2, m2);
                let f3 = self.add_face(i3, m3, m2);
                let f4 = self.add_face(m1, m2, m3);

                new_faces.extend(vec![face_index, f2, f3, f4]);
            }

            faces = new_faces;
        }
    }
}

#[derive(Default)]
pub struct MidpointIndexCache {
    map: HashMap<(u32, u32), u32>,
    keys: HashMap<u32, Vec<(u32, u32)>>,
}

impl MidpointIndexCache {
    fn insert(&mut self, i1: u32, i2: u32, vertex_index: u32) {
        let key = if i1 < i2 { (i1, i2) } else { (i2, i1) };

        self.map.insert(key, vertex_index);
        self.keys.entry(i1).or_insert_with(Vec::new).push(key);
        self.keys.entry(i2).or_insert_with(Vec::new).push(key);
    }

    fn get(&mut self, i1: u32, i2: u32) -> Option<&u32> {
        let key = if i1 < i2 { (i1, i2) } else { (i2, i1) };

        self.map.get(&key)
    }

    // TODO: remove function
}

#[derive(Default)]
pub struct UnusedVertices(Vec<u32>);

impl UnusedVertices {
    fn add(&mut self, vertex: u32) {
        self.0.push(vertex);
    }

    fn get(&mut self) -> Option<u32> {
        self.0.pop()
    }
}

pub struct UnusedIndices(Vec<usize>);

impl UnusedIndices {
    fn add(&mut self, vertex: usize) {
        self.0.push(vertex);
    }

    fn get(&mut self) -> Option<usize> {
        self.0.pop()
    }
}

impl VerticesExt for Vec<[f32; 3]> {
    fn add_vertex(&mut self, position: [f32; 3]) -> u32 {
        self.push(position);
        self.len() as u32 - 1
    }

    fn get_vertex(&self, index: u32) -> &[f32; 3] {
        self.get(index as usize)
            .expect("Vertex does not exist at that position")
    }
}

impl IndicesExt for Vec<u32> {
    fn add_face(&mut self, i1: u32, i2: u32, i3: u32) -> usize {
        self.extend(vec![i1, i2, i3]);
        self.len() - 3
    }

    fn replace_face(&mut self, position: usize, i1: u32, i2: u32, i3: u32) {
        self[position + 0] = i1;
        self[position + 1] = i2;
        self[position + 2] = i3;
    }

    fn get_face(&self, index: usize) -> (u32, u32, u32) {
        let face = self.get(index..index + 3).expect("Face does not exist");

        (face[0], face[1], face[2])
    }
}
