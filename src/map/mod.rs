use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use libnoise::prelude::*;

use self::heightmap::Heightmap;

mod heightmap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_test_environment);
    }
}

fn setup_test_environment(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    const MAP_SIZE: u32 = 500;

    let obstacle_model = PbrBundle {
        mesh: meshes.add(shape::Cube::default().into()),
        material: materials.add(StandardMaterial { ..default() }),
        ..Default::default()
    };

    const HALF_SIZE: f32 = MAP_SIZE as f32 / 2.0;
    const OBSTACLE_COUNT: u32 = MAP_SIZE / 10;

    commands
        .spawn((SpatialBundle::default(), Name::new("Obstacles")))
        .with_children(|parent| {
            for x in 0..=OBSTACLE_COUNT {
                for z in 0..=OBSTACLE_COUNT {
                    parent.spawn((
                        PbrBundle {
                            transform: Transform::from_xyz(
                                (x * 10) as f32 - HALF_SIZE,
                                0.5,
                                (z * 10) as f32 - HALF_SIZE,
                            )
                            .with_scale(Vec3::splat(0.5)),
                            ..obstacle_model.clone()
                        },
                        Name::new(format!("{}, {}", x, z)),
                    ));
                }
            }
        });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 4.0)),
        ..Default::default()
    });

    let terrain_mesh = generate_terrain();

    // ground plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(terrain_mesh),
            material: materials.add(Color::LIME_GREEN.into()),
            ..default()
        },
        Name::new("Terrain"),
    ));
}

fn generate_heightmap() -> Heightmap {
    let mut heightmap = Heightmap::new(256, 256);

    let generator = Source::simplex(42);

    for x in 0..256 {
        for z in 0..256 {
            heightmap.set(x, z, (generator.sample([x as f64, z as f64])) as i16);
        }
    }

    heightmap
}

#[derive(Clone, Copy)]
struct Tile {
    pub x: u16,
    pub z: u16,
    pub heights: [u16; 4],
}

impl Tile {
    fn new(x: u16, z: u16, generator: impl Generator2D) -> Tile {
        Tile {
            x,
            z,
            heights: Self::generate_height(x, z, generator),
        }
    }

    fn v0(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.heights[0] as f32, self.z as f32)
    }

    fn v1(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.heights[1] as f32, (self.z + 1) as f32)
    }

    fn v2(&self) -> Vec3 {
        Vec3::new(
            (self.x + 1) as f32,
            self.heights[2] as f32,
            (self.z + 1) as f32,
        )
    }

    fn v3(&self) -> Vec3 {
        Vec3::new(
            (self.x + 1) as f32,
            self.heights[3] as f32,
            (self.z + 1) as f32,
        )
    }

    fn generate_height(x: u16, z: u16, generator: impl Generator2D) -> [u16; 4] {
        [
            generator.sample([x as f64, z as f64]) as u16,
            generator.sample([x as f64 + 1.0, z as f64]) as u16,
            generator.sample([x as f64, z as f64 + 1.0]) as u16,
            generator.sample([x as f64 + 1.0, z as f64 + 1.0]) as u16,
        ]
    }

    fn append_vertices(&self, mut vertices: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        vertices.push([self.x as f32, self.heights[0] as f32, self.z as f32]);
        vertices.push([self.x as f32, self.heights[1] as f32, (self.z + 1) as f32]);
        vertices.push([
            (self.x + 1) as f32,
            self.heights[2] as f32,
            (self.z + 1) as f32,
        ]);
        vertices.push([(self.x + 1) as f32, self.heights[3] as f32, self.z as f32]);
        vertices
    }

    fn append_indices(&self, (next_index, mut indices): (u32, Vec<u32>)) -> (u32, Vec<u32>) {
        indices.push(next_index);
        indices.push(next_index + 1);
        indices.push(next_index + 2);

        indices.push(next_index + 2);
        indices.push(next_index + 3);
        indices.push(next_index);

        (next_index + 4, indices)
    }

    fn append_normals(&self, mut normals: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        let n0 = (self.v3() - self.v0())
            .cross(self.v1() - self.v0())
            .normalize();
        let n1 = (self.v0() - self.v1())
            .cross(self.v2() - self.v1())
            .normalize();
        let n2 = (self.v1() - self.v2())
            .cross(self.v3() - self.v2())
            .normalize();
        let n3 = (self.v2() - self.v3())
            .cross(self.v0() - self.v3())
            .normalize();

        normals.push(n0.to_array());
        normals.push(n1.to_array());
        normals.push(n2.to_array());
        normals.push(n3.to_array());

        normals
    }
}

fn generate_terrain() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let _heightmap = generate_heightmap();
    const MAP_SIZE: u16 = 128;

    let generator = Source::simplex(42);

    let tiles = (0..MAP_SIZE * MAP_SIZE)
        .map(|i| Tile::new(i / MAP_SIZE, i % MAP_SIZE, generator.clone()))
        .collect::<Vec<_>>();

    let vertices = tiles.iter().fold(Vec::new(), |v, t| t.append_vertices(v));
    let (_, indices) = tiles
        .iter()
        .fold((0, Vec::new()), |p, t| t.append_indices(p));
    let normals = tiles.iter().fold(Vec::new(), |v, t| t.append_normals(v));

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
