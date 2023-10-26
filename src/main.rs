use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_resource::PrimitiveTopology,
        settings::{Backends, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    DefaultPlugins,
};
use leafwing_input_manager::prelude::*;

mod fly_by_cam;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::VULKAN),
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            },
        }))
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_plugins(fly_by_cam::FlyByCameraPlugin)
        .add_systems(
            Update,
            (move_player, toggle_camera, bevy::window::close_on_esc),
        )
        .init_resource::<PlayerControllerConfig>()
        .add_systems(
            Startup,
            (setup_camera, setup_test_environment, spawn_player),
        )
        .run();
}

#[derive(Resource, Default)]
struct PlayerControllerConfig {
    active: bool,
}

#[derive(Actionlike, PartialEq, PartialOrd, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Capsule::default().into()),
            ..default()
        },
        InputManagerBundle::<Action> {
            input_map: InputMap::new([(
                VirtualDPad {
                    up: KeyCode::W.into(),
                    down: KeyCode::S.into(),
                    left: KeyCode::A.into(),
                    right: KeyCode::D.into(),
                },
                Action::Move,
            )]),
            ..default()
        },
        Player,
    ));
}

fn toggle_camera(
    input: Res<Input<KeyCode>>,
    mut player_controller_config: ResMut<PlayerControllerConfig>,
    mut camera_config: ResMut<fly_by_cam::FlyByCameraConfig>,
) {
    if input.just_released(KeyCode::F1) {
        player_controller_config.active = !player_controller_config.active;
        camera_config.active = !camera_config.active;
    }
}

fn move_player(
    mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>,
    player_controller_config: Res<PlayerControllerConfig>,
    time: Res<Time>,
) {
    if !player_controller_config.active {
        return;
    }

    let (state, mut transform) = query.single_mut();

    if state.pressed(Action::Move) {
        let axis_data = state.axis_pair(Action::Move).unwrap();
        let move_value: Vec2 = axis_data.into();
        let forward = transform.forward();
        let right = transform.right();

        transform.translation += move_value.x * forward * time.delta_seconds();
        transform.translation += move_value.y * right * time.delta_seconds();
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
        .spawn(SpatialBundle::default())
        .insert(Name::new(format!("Obstacles")))
        .with_children(|parent| {
            for x in 0..=OBSTACLE_COUNT {
                for z in 0..=OBSTACLE_COUNT {
                    parent
                        .spawn(PbrBundle {
                            transform: Transform::from_xyz(
                                (x * 10) as f32 - HALF_SIZE,
                                0.5,
                                (z * 10) as f32 - HALF_SIZE,
                            ),
                            ..obstacle_model.clone()
                        })
                        .insert(Name::new(format!("{}, {}", x, z)));
                }
            }
        });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.0)),
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

#[derive(Component, Debug, Clone, Copy)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::Y, Vec3::Y),
            ..default()
        },
        fly_by_cam::FlyByCamera,
        MainCamera,
    ));
}

#[derive(Clone, Copy)]
struct Tile {
    pub x: u16,
    pub z: u16,
    pub heights: [u16; 4],
}

impl Tile {
    fn new(x: u16, z: u16) -> Tile {
        Tile {
            x,
            z,
            heights: [
                Tile::generate_height(x, z),
                Tile::generate_height(x, z + 1),
                Tile::generate_height(x + 1, z + 1),
                Tile::generate_height(x + 1, z),
            ],
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

    fn generate_height(x: u16, z: u16) -> u16 {
        if x % 2 == 0 || z % 2 == 0 {
            0
        } else {
            1
        }
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

    const MAP_SIZE: u16 = 128;

    let tiles = (0..MAP_SIZE * MAP_SIZE)
        .map(|i| Tile::new(i / MAP_SIZE, i % MAP_SIZE))
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
