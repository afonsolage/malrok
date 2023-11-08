use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Resource, Reflect, InspectorOptions, Debug, Clone, Copy)]
#[reflect(Resource, InspectorOptions)]
pub struct HeightmapConfig {
    pub size: u16,
    pub height_scale: f32,
    pub size_scale: f32,
    pub octaves: u32,
    pub persistence: f64,
    pub frequency: f64,
    pub lacunarity: f64,
    pub seed: u64,
}

impl Default for HeightmapConfig {
    fn default() -> Self {
        HeightmapConfig {
            size: 256,
            height_scale: 50.0,
            size_scale: 5.0,
            octaves: 3,
            persistence: 0.16,
            frequency: 0.02,
            lacunarity: 1.0,
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
        (x * self.size + z) as usize
    }

    #[inline]
    fn position(&self, index: usize) -> (u16, u16) {
        (index as u16 / self.size, index as u16 % self.size)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Heightmap {
    pub config: HeightmapConfig,
    buffer: Vec<u8>,
}

impl Heightmap {
    pub fn new(config: HeightmapConfig) -> Self {
        Heightmap {
            config,
            buffer: vec![0; config.buffer_size()],
        }
    }

    pub fn get(&self, x: u16, z: u16) -> u8 {
        self.buffer[self.config.index(x, z)]
    }

    pub fn set(&mut self, x: u16, z: u16, value: u8) {
        let index = self.config.index(x, z);
        self.buffer[index] = value;
    }

    pub fn index(&self, x: u16, z: u16) -> usize {
        self.config.index(x, z)
    }

    pub fn position(&self, index: usize) -> (u16, u16) {
        self.config.position(index)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl std::ops::Index<usize> for Heightmap {
    type Output = u8;

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
    type Item = u8;

    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<'a> IntoIterator for &'a Heightmap {
    type Item = &'a u8;

    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl std::fmt::Display for Heightmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Heightmap [{:?}]({})", self.config, self.buffer.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_heightmap() {
        let config = HeightmapConfig {
            size: 128,
            ..Default::default()
        };
        let heightmap = Heightmap::new(config);

        assert_eq!(heightmap.buffer.len(), config.buffer_size());
    }

    #[test]
    fn test_get_set_height() {
        let config = HeightmapConfig::default();
        let mut heightmap = Heightmap::new(config);

        let x = 10;
        let z = 20;
        let value = 42;

        heightmap.set(x, z, value);
        assert_eq!(heightmap.get(x, z), value);
    }

    #[test]
    fn test_index_height() {
        let config = HeightmapConfig::default();
        let mut heightmap = Heightmap::new(config);

        let index = 42;
        let value = 55;

        heightmap[index] = value;
        assert_eq!(heightmap[index], value);
    }

    #[test]
    fn test_position_index_conversion() {
        let config = HeightmapConfig {
            size: 128,
            ..Default::default()
        };
        let heightmap = Heightmap::new(config);

        let x = 10;
        let z = 20;
        let index = heightmap.config.index(x, z);
        let (x_result, z_result) = heightmap.config.position(index);

        assert_eq!(x_result, x);
        assert_eq!(z_result, z);
    }
}
