use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages},
};
use chunk::Chunk;
use mesh::{MidpointIndexCache, UnusedIndices, UnusedVertices, VertexRc};

use super::orbit_camera::MainCamera;

mod chunk;
mod mesh;

#[derive(Bundle)]
pub struct PlanetViewBundle {
    pbr: PbrBundle,
    chunk: Chunk,
    midpoint_cache: mesh::MidpointIndexCache,
    unused_indices: mesh::UnusedIndices,
    unused_vertices: mesh::UnusedVertices,
    vertex_rc: mesh::VertexRc,
}

#[derive(Component)]
pub struct CurrentPlanet;

pub fn test_init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut vertices = vec![[-2.0, -1.0, 0.0], [0.0, 2.46, 0.0], [2.0, -1.0, 0.0]];
    let mut indices = vec![];
    let mut unused_indices = UnusedIndices::default();
    let mut unused_vertices = UnusedVertices::default();
    let mut vertex_rc = VertexRc::default();
    let mut midpoint_cache = MidpointIndexCache::default();

    let chunk = Chunk::new(
        true,
        1.0,
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
            pbr: PbrBundle {
                mesh: meshes.add(mesh),
                ..default()
            },
            chunk,
            midpoint_cache,
            unused_indices,
            unused_vertices,
            vertex_rc,
        },
        CurrentPlanet,
        Wireframe,
    ));
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
    let (
        mesh_handle,
        mut chunk,
        mut midpoint_cache,
        mut unused_indices,
        mut unused_vertices,
        mut vertex_rc,
    ) = query.single_mut();

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

    println!("vertices: {}", vertices.len());
    println!("faces: {}", indices.len() / 3);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(Indices::U32(indices));
}
