use libnoise::{Generator, Source};

use super::heightmap::{Heightmap, HeightmapSettings};

pub fn generate_terrain(settings: &HeightmapSettings) -> Heightmap {
    let mut heightmap = Heightmap::new(settings.width, settings.depth);
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
