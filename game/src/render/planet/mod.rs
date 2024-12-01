use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages},
};

use chunk::Chunk;
use mesh::{MidpointIndexCache, UnusedIndices, UnusedVertices, VertexRc};

use super::MainCamera;

const PHI: f32 = 1.618033988749894848204586834365638118_f32;
const ICOSAHEDRON_VERTEX_POSITIONS: [([f32; 3], [f32; 3], [f32; 3]); 20] = [
    ([PHI, 1.0, 0.0], [0.0, PHI, 1.0], [1.0, 0.0, PHI]),
    ([PHI, 1.0, 0.0], [1.0, 0.0, -PHI], [0.0, PHI, -1.0]),
    ([PHI, -1.0, 0.0], [1.0, 0.0, PHI], [0.0, -PHI, 1.0]),
    ([PHI, -1.0, 0.0], [0.0, -PHI, -1.0], [1.0, 0.0, -PHI]),
    ([-PHI, 1.0, 0.0], [-1.0, 0.0, PHI], [0.0, PHI, 1.0]),
    ([-PHI, 1.0, 0.0], [0.0, PHI, -1.0], [-1.0, 0.0, -PHI]),
    ([-PHI, -1.0, 0.0], [0.0, -PHI, 1.0], [-1.0, 0.0, PHI]),
    ([-PHI, -1.0, 0.0], [-1.0, 0.0, -PHI], [0.0, -PHI, -1.0]),
    ([PHI, 1.0, 0.0], [1.0, 0.0, PHI], [PHI, -1.0, 0.0]),
    ([PHI, 1.0, 0.0], [PHI, -1.0, 0.0], [1.0, 0.0, -PHI]),
    ([-PHI, 1.0, 0.0], [-PHI, -1.0, 0.0], [-1.0, 0.0, PHI]),
    ([-PHI, 1.0, 0.0], [-1.0, 0.0, -PHI], [-PHI, -1.0, 0.0]),
    ([1.0, 0.0, PHI], [0.0, PHI, 1.0], [-1.0, 0.0, PHI]),
    ([1.0, 0.0, -PHI], [-1.0, 0.0, -PHI], [0.0, PHI, -1.0]),
    ([1.0, 0.0, PHI], [-1.0, 0.0, PHI], [0.0, -PHI, 1.0]),
    ([1.0, 0.0, -PHI], [0.0, -PHI, -1.0], [-1.0, 0.0, -PHI]),
    ([0.0, PHI, 1.0], [0.0, PHI, -1.0], [-PHI, 1.0, 0.0]),
    ([0.0, PHI, -1.0], [0.0, PHI, 1.0], [PHI, 1.0, 0.0]),
    ([0.0, -PHI, 1.0], [0.0, -PHI, -1.0], [PHI, -1.0, 0.0]),
    ([0.0, -PHI, -1.0], [0.0, -PHI, 1.0], [-PHI, -1.0, 0.0]),
];

mod chunk;
pub mod material;
mod mesh;

#[derive(Bundle)]
pub struct PlanetViewBundle {
    mmb: MaterialMeshBundle<material::PlanetMaterial>,
    chunk: Chunk,
    midpoint_cache: mesh::MidpointIndexCache,
    unused_indices: mesh::UnusedIndices,
    unused_vertices: mesh::UnusedVertices,
    vertex_rc: mesh::VertexRc,
}

#[derive(Component)]
pub struct Planet {
    pub radius: f32,
}

#[derive(Component)]
pub struct CurrentPlanet;

pub fn update_chunks(
    mut query: Query<(
        &Mesh3d,
        &mut Chunk,
        &mut MidpointIndexCache,
        &mut UnusedIndices,
        &mut UnusedVertices,
        &mut VertexRc,
    )>,
    cammera_query: Query<&Transform, With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (
        mesh_handle,
        mut chunk,
        mut midpoint_cache,
        mut unused_indices,
        mut unused_vertices,
        mut vertex_rc,
    ) in query.iter_mut()
    {
        let mesh = meshes.get_mut(mesh_handle).unwrap();

        let bevy::render::mesh::VertexAttributeValues::Float32x3(mut vertices) =
            mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!("Chunk does not have vertex positions")
        };
        let Indices::U32(mut indices) = mesh.remove_indices().unwrap() else {
            panic!("Chunk does not have faces")
        };

        let chunk = chunk.as_mut();
        let cammera_position = cammera_query.single().translation;
        // Habra que pasarlo a coordenadas relativas respecto al centro del planeta
        let camera_position = [cammera_position.x, cammera_position.y, cammera_position.z];

        chunk.divide_or_undivide(
            &mut indices,
            &mut vertices,
            unused_indices.as_mut(),
            unused_vertices.as_mut(),
            vertex_rc.as_mut(),
            midpoint_cache.as_mut(),
            &camera_position,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
    }
}

pub fn on_planet_unload(
    mut commands: Commands,
    mut planet_to_unload: RemovedComponents<CurrentPlanet>,
    planets: Query<&Planet>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for planet in planet_to_unload.read() {
        let planet_config = planets
            .get(planet)
            .expect("Planet must have the planet component");

        let mesh = Mesh3d::from(meshes.add(Sphere::new(planet_config.radius)));
        let low_res_view = commands
            .spawn(PbrBundle {
                mesh: mesh,
                ..default()
            })
            .id();

        commands
            .get_entity(planet)
            .unwrap()
            .despawn_descendants()
            .add_child(low_res_view);
    }
}

pub fn on_planet_load(
    mut commands: Commands,
    planet_to_load: Query<(Entity, &Planet), Added<CurrentPlanet>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<material::PlanetMaterial>>,
) {
    for (entity, planet) in planet_to_load.iter() {
        // Spawn all the chunks
        let mut chunks = Vec::with_capacity(20);
        for (v1, v2, v3) in ICOSAHEDRON_VERTEX_POSITIONS {
            let mut vertices = vec![v1, v2, v3];
            for vertex in &mut vertices {
                for coord in vertex.iter_mut() {
                    *coord *= planet.radius;
                }
            }
            let mut indices = vec![];
            let mut unused_indices = UnusedIndices::default();
            let mut unused_vertices = UnusedVertices::default();
            let mut vertex_rc = VertexRc::default();
            let mut midpoint_cache = MidpointIndexCache::default();

            let chunk = Chunk::new(
                true,
                1,
                planet.radius,
                [0, 1, 2],
                &mut indices,
                &mut vertices,
                &mut unused_indices,
                &mut unused_vertices,
                &mut vertex_rc,
                &mut midpoint_cache,
            );

            let mesh = Mesh::new(
                bevy::render::mesh::PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_indices(Indices::U32(indices));

            let chunk = commands
                .spawn(PlanetViewBundle {
                    mmb: MaterialMeshBundle {
                        mesh: Mesh3d::from(meshes.add(mesh)),
                        material: MeshMaterial3d::from(materials.add(material::PlanetMaterial {
                            radius: planet.radius,
                            deviation: 8800.0,
                        })),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                        ..default()
                    },
                    chunk,
                    midpoint_cache,
                    unused_indices,
                    unused_vertices,
                    vertex_rc,
                })
                .id();
            chunks.push(chunk);
        }

        let mut entity = commands.get_entity(entity).unwrap();
        entity.despawn_descendants();
        for chunk in chunks {
            entity.add_child(chunk);
        }
    }
}
