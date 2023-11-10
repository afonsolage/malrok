use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{ImageSampler, ImageSamplerDescriptor},
    },
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use self::heightmap::{Heightmap, HeightmapConfig};

mod generator;
mod heightmap;
mod layered_heightmap;
mod mesher;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_test_environment)
            .init_resource::<HeightmapConfig>()
            .add_plugins(ResourceInspectorPlugin::<HeightmapConfig>::default())
            .add_systems(
                Update,
                generate_heightmap.run_if(resource_changed::<HeightmapConfig>()),
            );
    }
}

#[derive(Component)]
struct HeightmapMarker;

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
}

impl From<Heightmap> for Image {
    fn from(heightmap: Heightmap) -> Self {
        let size = heightmap.config.size as u32;
        let data = heightmap
            .into_iter()
            .flat_map(|h| [h, h, h, 255])
            .collect::<Vec<_>>();

        Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: size,
                    height: size,
                    ..default()
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            data,
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor::nearest()),
            ..default()
        }
    }
}

fn generate_heightmap(
    mut commands: Commands,
    config: Res<HeightmapConfig>,
    q_existing_heightmap: Query<Entity, With<HeightmapMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let heightmap = generator::generate(*config);

    for entity in &q_existing_heightmap {
        commands.entity(entity).despawn_recursive();
    }

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(heightmap.config.size as f32).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(images.add(heightmap.clone().into())),
                unlit: false,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.1, 1.5),
            ..default()
        },
        Name::new("Heightmap texture"),
        HeightmapMarker,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(heightmap.into()),
            material: materials.add(Color::LIME_GREEN.into()),
            ..default()
        },
        Name::new("Terrain"),
        HeightmapMarker,
    ));
}
