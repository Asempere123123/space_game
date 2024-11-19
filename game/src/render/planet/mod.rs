use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
    },
};
use chunk::Chunk;
use mesh::{IndicesExt, MidpointIndexCache, UnusedIndices, UnusedVertices, VertexRc, VerticesExt};

mod chunk;
mod mesh;

#[derive(Bundle, Default)]
pub struct PlanetViewBundle {
    pbr: PbrBundle,
    midpoint_cache: mesh::MidpointIndexCache,
    unused_indices: mesh::UnusedIndices,
    unused_vertices: mesh::UnusedVertices,
    vertex_rc: mesh::VertexRc,
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
        vec![[-2.0, -1.0, 0.0], [0.0, 2.46, 0.0], [2.0, -1.0, 0.0]],
    )
    .with_inserted_indices(Indices::U32(vec![]));

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
    mut query: Query<
        (
            &Handle<Mesh>,
            &mut MidpointIndexCache,
            &mut UnusedIndices,
            &mut UnusedVertices,
            &mut VertexRc,
        ),
        With<CurrentPlanet>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (mesh_handle, mut midpoint_cache, mut unused_indices, mut unused_vertices, mut vertex_rc) =
        query.single_mut();
    let mesh = meshes.get_mut(mesh_handle).unwrap();

    let bevy::render::mesh::VertexAttributeValues::Float32x3(mut vertices) =
        mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
    else {
        panic!()
    };
    let Indices::U32(mut indices) = mesh.remove_indices().unwrap() else {
        panic!()
    };

    //indices.subdivide_face(cache, &mut vertices, 0, 5);
    let mut chunk = Chunk::new(
        (0.0, 0.0, 0.0),
        true,
        [0, 1, 2],
        &mut indices,
        &mut vertices,
        unused_indices.as_mut(),
        unused_vertices.as_mut(),
        vertex_rc.as_mut(),
        midpoint_cache.as_mut(),
    );

    chunk.subdivide(
        &mut indices,
        &mut vertices,
        unused_indices.as_mut(),
        unused_vertices.as_mut(),
        vertex_rc.as_mut(),
        midpoint_cache.as_mut(),
    );
    chunk.undivide(
        &mut indices,
        &mut vertices,
        unused_indices.as_mut(),
        unused_vertices.as_mut(),
        vertex_rc.as_mut(),
        midpoint_cache.as_mut(),
    );
    chunk.subdivide(
        &mut indices,
        &mut vertices,
        unused_indices.as_mut(),
        unused_vertices.as_mut(),
        vertex_rc.as_mut(),
        midpoint_cache.as_mut(),
    );

    // Central chunk
    chunk.children[1].subdivide(
        &mut indices,
        &mut vertices,
        unused_indices.as_mut(),
        unused_vertices.as_mut(),
        vertex_rc.as_mut(),
        midpoint_cache.as_mut(),
    );
    chunk.children[1].undivide(
        &mut indices,
        &mut vertices,
        unused_indices.as_mut(),
        unused_vertices.as_mut(),
        vertex_rc.as_mut(),
        midpoint_cache.as_mut(),
    );
    chunk.children[2].subdivide(
        &mut indices,
        &mut vertices,
        unused_indices.as_mut(),
        unused_vertices.as_mut(),
        vertex_rc.as_mut(),
        midpoint_cache.as_mut(),
    );

    println!("vertices: {}", vertices.len());
    println!("faces: {}", indices.len() / 3);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(Indices::U32(indices));
}
