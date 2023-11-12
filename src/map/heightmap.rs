use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
//
// A |
// M |           ____
// P |          /    \
// L |         /      \
// I |  __    /        \
// T | /  \  /          \
// U |/    --            \__/
// D +-----------------------
// E         FREQUENCY
//
#[derive(Resource, Reflect, InspectorOptions, Debug, Clone, Copy)]
#[reflect(Resource, InspectorOptions)]
pub struct HeightmapConfig {
    pub size: u16,
    // Numbers of noise levels to use
    pub octaves: u32,
    // Increase of frequency in each octave, must be greater than 1
    pub lacunarity: f64,
    // Decrease of amplitude in each octave, must be in range [0, 1]
    pub persistence: f64,
    // Initial frequency
    pub frequency: f64,
    pub seed: u64,
}

impl Default for HeightmapConfig {
    fn default() -> Self {
        HeightmapConfig {
            size: 256,
            octaves: 6,
            persistence: 0.5,
            frequency: 1.0,
            lacunarity: 2.0,
            seed: 42,
        }
    }
}

impl HeightmapConfig {
    #[inline]
    fn buffer_size(&self) -> usize {
        self.size as usize * self.size as usize
    }

    #[inline]
    fn index(&self, x: u16, z: u16) -> usize {
        x as usize * self.size as usize + z as usize
    }

    #[inline]
    fn position(&self, index: usize) -> [u16; 2] {
        [index as u16 / self.size, index as u16 % self.size]
    }
}

#[derive(Default, Debug, Clone)]
pub struct Heightmap {
    pub config: HeightmapConfig,
    buffer: Vec<f32>,
}

impl Heightmap {
    pub fn new(config: HeightmapConfig) -> Self {
        Heightmap {
            config,
            buffer: vec![0.0; config.buffer_size()],
        }
    }

    pub fn get(&self, x: u16, z: u16) -> f32 {
        self.buffer[self.config.index(x, z)]
    }

    pub fn set(&mut self, x: u16, z: u16, value: f32) {
        let index = self.config.index(x, z);
        self.buffer[index] = value;
    }

    pub fn index(&self, x: u16, z: u16) -> usize {
        self.config.index(x, z)
    }

    pub fn position(&self, index: usize) -> [u16; 2] {
        self.config.position(index)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl std::ops::Index<usize> for Heightmap {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl std::ops::IndexMut<usize> for Heightmap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

impl IntoIterator for Heightmap {
    type Item = f32;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<'a> IntoIterator for &'a Heightmap {
    type Item = &'a f32;

    type IntoIter = std::slice::Iter<'a, f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl std::fmt::Display for Heightmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Heightmap [{:?}]({})", self.config, self.buffer.len())
    }
}
