use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

use super::heightmap::Heightmap;

impl From<Heightmap> for Mesh {
    fn from(heightmap: Heightmap) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let vertices = calc_vertices(heightmap);

        let indices = calc_indices(&vertices);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

        mesh
    }
}

fn calc_indices(vertices: &Vec<[f32; 3]>) -> Vec<u32> {
    let mut indices = vec![];
    for index in (0..vertices.len()).step_by(4) {
        let index = index as u32;
        indices.push(index);
        indices.push(index + 1);
        indices.push(index + 2);

        indices.push(index + 1);
        indices.push(index + 3);
        indices.push(index + 2);
    }
    indices
}

#[inline]
fn calc_vertice_at(x: u16, z: u16, heightmap: &Heightmap) -> [f32; 3] {
    let height = heightmap.get(x, z);
    [x as f32, height as f32, z as f32]
}

fn calc_vertices(heightmap: Heightmap) -> Vec<[f32; 3]> {
    let mut vertices = vec![];
    let size = heightmap.config.size - 1;
    for x in 0..size {
        for z in 0..size {
            let v0 = calc_vertice_at(x, z, &heightmap);
            let v1 = calc_vertice_at(x, z + 1, &heightmap);
            let v2 = calc_vertice_at(x + 1, z, &heightmap);
            let v3 = calc_vertice_at(x + 1, z + 1, &heightmap);
            vertices.push(v0);
            vertices.push(v1);
            vertices.push(v2);
            vertices.push(v3);
        }
    }
    vertices
}
