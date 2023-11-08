use libnoise::{Generator, Source};

use super::heightmap::{Heightmap, HeightmapConfig};

pub fn generate(config: HeightmapConfig) -> Heightmap {
    let mut heightmap = Heightmap::new(config);

    generate_terrain(&mut heightmap);

    heightmap
}

fn generate_terrain(heightmap: &mut Heightmap) {
    let HeightmapConfig {
        height_scale,
        octaves,
        frequency,
        lacunarity,
        persistence,
        seed,
        ..
    } = heightmap.config;

    let generator = Source::simplex(seed).fbm(octaves, frequency, lacunarity, persistence);

    for i in 0..heightmap.len() {
        let (x, z) = heightmap.position(i);
        let height = (generator.sample([x as f64, z as f64]) + 1.0) / 2.0;
        heightmap[i] = (height * height_scale as f64) as u8;
    }
}
