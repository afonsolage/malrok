use libnoise::{Generator, Source};

use super::heightmap::{Heightmap, HeightmapConfig};

pub fn generate(config: HeightmapConfig) -> Heightmap {
    let mut heightmap = Heightmap::new(config);

    generate_terrain(&mut heightmap);

    heightmap
}

fn generate_terrain(heightmap: &mut Heightmap) {
    let HeightmapConfig {
        octaves,
        frequency,
        lacunarity,
        persistence,
        seed,
        ..
    } = heightmap.config;

    let generator = Source::simplex(seed).fbm(octaves, frequency, lacunarity, persistence);

    for i in 0..heightmap.len() {
        let point = heightmap
            .position(i)
            .map(|v| v as f64 / heightmap.config.size as f64);
        let height = (generator.sample(point) + 1.0) / 2.0;
        heightmap[i] = height as f32;
    }
}
