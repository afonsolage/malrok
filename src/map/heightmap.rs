use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Resource, Debug, InspectorOptions, Reflect, Clone, Copy)]
#[reflect(Resource, Default, Debug)]
pub struct HeightmapSettings {
    pub width: u16,
    pub depth: u16,
    pub seed: u64,
    // Numbers of noise levels to use
    pub octaves: u32,
    // Increase of frequency in each octave, must be greater than 1
    pub lacunarity: f64,
    // Decrease of amplitude in each octave, must be in range [0, 1]
    pub persistence: f64,
    // Initial frequency
    pub frequency: f64,
}

impl HeightmapSettings {
    pub fn new(width: u16, depth: u16) -> Self {
        Self {
            width,
            depth,
            ..Default::default()
        }
    }
}

impl Default for HeightmapSettings {
    fn default() -> Self {
        Self {
            width: 256,
            depth: 256,
            seed: 42,
            octaves: 5,
            persistence: 0.5,
            frequency: 1.0,
            lacunarity: 2.0,
        }
    }
}

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
#[derive(Resource, Reflect, InspectorOptions, Debug, Clone)]
#[reflect(Resource, Default, InspectorOptions)]
pub struct Heightmap {
    pub width: u16,
    pub depth: u16,
    #[reflect(ignore)]
    buffer: Vec<f32>,
    pub image: Handle<Image>,
}

impl Heightmap {
    pub fn new(width: u16, depth: u16) -> Self {
        Heightmap {
            width,
            depth,
            buffer: vec![0.0; width as usize * depth as usize],
            image: default(),
        }
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    pub fn index(&self, x: u16, z: u16) -> usize {
        x as usize * self.depth as usize + z as usize
    }

    #[inline]
    pub fn position(&self, index: usize) -> [u16; 2] {
        [index as u16 / self.depth, index as u16 % self.depth]
    }

    pub fn get(&self, x: u16, z: u16) -> f32 {
        self.buffer[self.index(x, z)]
    }

    pub fn set(&mut self, x: u16, z: u16, value: f32) {
        let index = self.index(x, z);
        self.buffer[index] = value;
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

impl Default for Heightmap {
    fn default() -> Self {
        let width = 256;
        let depth = 256;
        Self {
            buffer: vec![0.0; width as usize * depth as usize],
            width,
            depth,
            image: default(),
        }
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
