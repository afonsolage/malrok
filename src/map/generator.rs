use libnoise::{Generator, Source};

use super::{
    heightmap::{Heightmap, HeightmapSettings},
    HeightmapLayers,
};

pub fn generate_terrain(settings: &HeightmapSettings) -> Heightmap {
    let mut heightmap = Heightmap::new(settings.name.clone(), settings.width, settings.depth);
    let generator = Source::simplex(settings.seed).fbm(
        settings.octaves,
        settings.frequency,
        settings.lacunarity,
        settings.persistence,
    );

    for i in 0..heightmap.buffer_size() {
        let [x, z] = heightmap.position(i);
        let point = [
            x as f64 / heightmap.width as f64,
            z as f64 / heightmap.depth as f64,
        ];
        let height = (generator.sample(point) + 1.0) / 2.0;
        heightmap[i] = height as f32;
    }

    heightmap
}

pub fn combine_heightmap_layers(layers: &HeightmapLayers) -> Heightmap {
    let mut combined_heightmap = Heightmap::new("Final", 256, 256);

    for heightmap in layers.iter() {
        for (index, height) in heightmap.into_iter().enumerate() {
            combined_heightmap[index] = (combined_heightmap[index] + height) / 2.0;
        }
    }

    combined_heightmap
}
