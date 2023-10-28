use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::fly_by_cam::FlyByCameraConfig;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Update, (move_player, toggle_camera))
            .add_systems(Startup, spawn_player)
            .init_resource::<PlayerControllerConfig>();
    }
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
    mut camera_config: ResMut<FlyByCameraConfig>,
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
