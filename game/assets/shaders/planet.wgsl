#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

@group(2) @binding(0) var<uniform> radius: f32;
@group(2) @binding(1) var depth_texture: texture_2d<f32>;
@group(2) @binding(2) var depth_sampler: sampler;
@group(2) @binding(3) var<uniform> deviation: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

@vertex
fn vertex(
    vertex: Vertex
) -> VertexOutput {
    var out: VertexOutput;

    var normalized_position = normalize(vertex.position);

    // Calculate offset
    var lat = asin(normalized_position.z);
    var long = atan2(normalized_position.y, normalized_position.x);
    var offset = textureGather(0, depth_texture, depth_sampler, vec2<f32>(lat, long)).r * deviation;

    var position = normalized_position * (radius + offset);

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(position, 1.0)
    );
    // Just color it by height
    out.color = textureGather(0, depth_texture, depth_sampler, vec2<f32>(lat, long));

    return out;
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@fragment
fn fragment(
    vertex_output: VertexOutput
) -> @location(0) vec4<f32> {
    return vertex_output.color;
}
