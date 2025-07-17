use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub data: PlanetUniforms,
}

#[derive(Clone, Copy, ShaderType, Debug)]
pub struct PlanetUniforms {
    pub radius: f32,
    pub deviation: f32,
    _align: f32,
    _align2: f32,
}

impl PlanetUniforms {
    pub fn new(radius: f32, deviation: f32) -> Self {
        Self {
            radius,
            deviation,
            _align: 0.,
            _align2: 0.,
        }
    }
}

impl Material for PlanetMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}
