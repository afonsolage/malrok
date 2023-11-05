use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use libnoise::prelude::*;

use self::heightmap::Heightmap;

mod generator;
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
    let mut heightmap = Heightmap::new(default());

    let generator = Source::simplex(42);

    for x in 0..256 {
        for z in 0..256 {
            let sample = (generator.sample([x as f64, z as f64]) + 1.0) / 2.0;
            let height = (sample * 10.0) as u8;
            heightmap.set(x, z, height);
        }
    }

    heightmap
}

impl From<Heightmap> for Mesh {
    fn from(value: Heightmap) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut vertices = vec![];
        for x in 0..255 {
            for z in 0..255 {
                let v0 = [x as f32, value.get(x, z) as f32, z as f32];
                let v1 = [x as f32, value.get(x, z + 1) as f32, (z + 1) as f32];
                let v2 = [(x + 1) as f32, value.get(x + 1, z) as f32, z as f32];
                let v3 = [
                    (x + 1) as f32,
                    value.get(x + 1, z + 1) as f32,
                    (z + 1) as f32,
                ];
                vertices.push(v0);
                vertices.push(v1);
                vertices.push(v2);
                vertices.push(v3);
            }
        }

        let mut indices = vec![];
        let mut index = 0;
        for _ in &vertices {
            indices.push(index);
            indices.push(index + 1);
            indices.push(index + 2);

            indices.push(index + 1);
            indices.push(index + 3);
            indices.push(index + 2);

            index += 4;
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

        mesh
    }
}

fn generate_terrain() -> Mesh {
    let heightmap = generate_heightmap();

    heightmap.into()
}
