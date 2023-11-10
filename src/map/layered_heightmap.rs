use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Resource, Reflect, InspectorOptions, Debug, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct LayeredHeightmapConfig {
    pub size: u16,
    pub seed: u64,
    pub layers: Vec<LayerConfig>,
}

impl Default for LayeredHeightmapConfig {
    fn default() -> Self {
        LayeredHeightmapConfig {
            size: 256,
            seed: 42,
            layers: default(),
        }
    }
}

#[derive(Resource, Reflect, InspectorOptions, Debug, Clone, Copy)]
#[reflect(Resource, InspectorOptions)]
pub struct LayerConfig {
    pub height_scale: f32,
    pub size_scale: f32,
    pub octaves: u32,
    pub persistence: f64,
    pub frequency: f64,
    pub lacunarity: f64,
}

impl Default for LayerConfig {
    fn default() -> Self {
        LayerConfig {
            height_scale: 50.0,
            size_scale: 5.0,
            octaves: 3,
            persistence: 0.16,
            frequency: 0.02,
            lacunarity: 1.0,
        }
    }
}
