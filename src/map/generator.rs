use libnoise::{Generator, Source};

use super::heightmap::Heightmap;

pub fn generate_terrain(heightmap: &mut Heightmap) {
    let generator = Source::simplex(heightmap.seed).fbm(
        heightmap.octaves,
        heightmap.frequency,
        heightmap.lacunarity,
        heightmap.persistence,
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
}
