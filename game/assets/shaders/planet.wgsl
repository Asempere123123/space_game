#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

@group(2) @binding(0) var<uniform> radius: f32;
@group(2) @binding(1) var<uniform> deviation: f32;

const pi = radians(180.0);

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
    var lat = (-asin(normalized_position.y) + pi/2.0) / pi;
    var long = (atan2(normalized_position.x, normalized_position.z) + pi) / (2.0*pi);
    var map_position = vec2<f32>(long, lat);

    var offset = calculate_offset(map_position);
    out.offset = offset;

    var position = normalized_position * (radius + offset * deviation);

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(position, 1.0)
    );

    return out;
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) offset: f32,
};

@fragment
fn fragment(
    vertex_output: VertexOutput
) -> @location(0) vec4<f32> {
    if (vertex_output.offset < -0.5) {
        return vec4(0.0, 0.0, 1.0, 0.3);  // Deep water
    } else if (vertex_output.offset >= -0.5 && vertex_output.offset < 0.0) {
        return vec4(0.0, 0.0, 1.0, 1.0);  // Water (blue)
    } else if (vertex_output.offset >= 0.0 && vertex_output.offset < 0.0005) {
        return vec4(1.0, 0.9, 0.6, 1.0);  // Sand (light yellowish)
    } else if (vertex_output.offset >= 0.0005 && vertex_output.offset < 0.25) {
        return vec4(0.0, 1.0, 0.0, 1.0);  // Grass (green)
    } else if (vertex_output.offset >= 0.25 && vertex_output.offset < 0.5) {
        return vec4(0.5, 0.5, 0.5, 1.0);  // Mountains (gray)
    } else {
        return vec4(1.0, 1.0, 1.0, 1.0);  // Snow
    }
}

fn permute4(x: vec4f) -> vec4f { return ((x * 34. + 1.) * x) % vec4f(289.); }
fn fade2(t: vec2f) -> vec2f { return t * t * t * (t * (t * 6. - 15.) + 10.); }

// https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
fn perlinNoise2(map_position: vec2f, scale: f32) -> f32 {
    var P = map_position * scale;

    var Pi: vec4f = floor(P.xyxy) + vec4f(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4f(0., 0., 1., 1.);
    Pi = Pi % vec4f(scale); // To avoid truncation effects in permutation
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4f = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2f = vec2f(gx.x, gy.x);
    var g10: vec2f = vec2f(gx.y, gy.y);
    var g01: vec2f = vec2f(gx.z, gy.z);
    var g11: vec2f = vec2f(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 *
        vec4f(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;
    let n00 = dot(g00, vec2f(fx.x, fy.x));
    let n10 = dot(g10, vec2f(fx.y, fy.y));
    let n01 = dot(g01, vec2f(fx.z, fy.z));
    let n11 = dot(g11, vec2f(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2f(n00, n01), vec2f(n10, n11), vec2f(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}

fn calculate_offset(map_position: vec2<f32>) -> f32 {
    let base_scale = 12.0;
    let octaves = 10u;
    let base_weight = 1.0;

    var noise = perlinNoise2(map_position, base_scale) * base_weight;

    // First pass, continents
    var scale = base_scale * 3.0;
    var weight = base_weight * 0.5;
    for (var i: u32 = 1u; i < octaves; i = i + 1u) {
        noise += perlinNoise2(map_position, scale) * weight;

        scale *= 2.0;
        weight *= 0.5;
    }

    // Second pass, mountains...
    let second_pass_octaves = 2u;
    scale = base_scale * pow(2.0, 4.0);
    weight = base_weight * (1.0/2.0);
    for (var i: u32 = 1u; i < second_pass_octaves; i = i + 1u) {
        noise += abs(perlinNoise2(map_position, scale) * weight);

        scale *= 2.0;
        weight *= 0.5;
    }

    return noise;
}
