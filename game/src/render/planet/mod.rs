use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
    },
};
use mesh::{IndicesExt, VerticesExt};

mod chunk;
mod mesh;

#[derive(Bundle, Default)]
pub struct PlanetViewBundle {
    pbr: PbrBundle,
    planet: Planet,
}

#[derive(Component, Default)]
pub struct Planet {
    midpoint_cache: mesh::MidpointIndexCache,
    unused_vertices: mesh::UnusedVertices,
}

#[derive(Component)]
pub struct CurrentPlanet;

pub fn test_init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-2.0, -2.0, 0.0], [0.0, 2.0, 0.0], [2.0, -2.0, 0.0]],
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2]));

    commands.spawn((
        PlanetViewBundle {
            pbr: PbrBundle {
                mesh: meshes.add(mesh),
                ..default()
            },
            ..default()
        },
        CurrentPlanet,
        Wireframe,
    ));
}

pub fn test_update(
    mut query: Query<(&Handle<Mesh>, &mut Planet), With<CurrentPlanet>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (mesh_handle, mut planet) = query.single_mut();
    let mesh = meshes.get_mut(mesh_handle).unwrap();

    let bevy::render::mesh::VertexAttributeValues::Float32x3(mut vertices) =
        mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
    else {
        panic!()
    };
    let Indices::U32(mut indices) = mesh.remove_indices().unwrap() else {
        panic!()
    };
    let cache = &mut planet.midpoint_cache;

    indices.subdivide_face(cache, &mut vertices, 0, 5);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(Indices::U32(indices));
}
