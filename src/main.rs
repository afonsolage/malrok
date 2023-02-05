use bevy::{prelude::*, DefaultPlugins};
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_system(move_player)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_test_environment)
        .add_startup_system(spawn_player)
        .run();
}

#[derive(Actionlike, PartialEq, PartialOrd, Clone, Copy, Hash, Debug)]
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

fn move_player(
    mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (state, mut transform) = query.single_mut();

    if state.pressed(Action::Move) {
        let axis_data = state.axis_pair(Action::Move).unwrap();
        let move_value: Vec2 = axis_data.into();

        transform.translation.x += move_value.x * time.delta_seconds();
        transform.translation.z += move_value.y * time.delta_seconds();
    }
}

fn setup_test_environment(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    const X_EXTENT: f32 = 14.0;

    let debug_material = materials.add(StandardMaterial { ..default() });

    let shapes = [
        meshes.add(shape::Cube::default().into()),
        meshes.add(shape::Box::default().into()),
        meshes.add(shape::Torus::default().into()),
        meshes.add(shape::Icosphere::default().try_into().unwrap()),
        meshes.add(shape::UVSphere::default().into()),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                2.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 4.)),
            ..default()
        },));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 50. }.into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::Y, Vec3::Y),
        ..default()
    });
}
