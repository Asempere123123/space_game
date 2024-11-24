use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages},
};

use chunk::Chunk;
use mesh::{MidpointIndexCache, UnusedIndices, UnusedVertices, VertexRc};

use super::orbit_camera::MainCamera;

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
pub struct CurrentPlanet;

pub fn test_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<material::PlanetMaterial>>,
) {
    let radius = 6378000.0;

    for (v1, v2, v3) in ICOSAHEDRON_VERTEX_POSITIONS {
        let mut vertices = vec![v1, v2, v3];
        let mut indices = vec![];
        let mut unused_indices = UnusedIndices::default();
        let mut unused_vertices = UnusedVertices::default();
        let mut vertex_rc = VertexRc::default();
        let mut midpoint_cache = MidpointIndexCache::default();

        let chunk = Chunk::new(
            true,
            1,
            radius,
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

        commands.spawn((
            PlanetViewBundle {
                mmb: MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(material::PlanetMaterial {
                        radius,
                        deviation: 8800.0,
                    }),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                    ..default()
                },
                chunk,
                midpoint_cache,
                unused_indices,
                unused_vertices,
                vertex_rc,
            },
            CurrentPlanet,
        ));
    }
}

pub fn test_update(
    mut query: Query<
        (
            &Handle<Mesh>,
            &mut Chunk,
            &mut MidpointIndexCache,
            &mut UnusedIndices,
            &mut UnusedVertices,
            &mut VertexRc,
        ),
        With<CurrentPlanet>,
    >,
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
            panic!()
        };
        let Indices::U32(mut indices) = mesh.remove_indices().unwrap() else {
            panic!()
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
