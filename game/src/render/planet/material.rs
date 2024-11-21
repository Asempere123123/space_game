use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub radius: f32,
    #[texture(1)]
    #[sampler(2)]
    pub depth: Handle<Image>,
    #[uniform(3)]
    pub deviation: f32,
}

impl Material for PlanetMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}
