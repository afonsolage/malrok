use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    DefaultPlugins,
};
use leafwing_input_manager::prelude::*;

mod fly_by_cam;
mod map;

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
        .add_plugins((fly_by_cam::FlyByCameraPlugin, map::MapPlugin))
        .add_systems(
            Update,
            (move_player, toggle_camera, bevy::window::close_on_esc),
        )
        .init_resource::<PlayerControllerConfig>()
        .add_systems(Startup, (setup_camera, spawn_player))
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
