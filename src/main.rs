use bevy::{
    app::AppExit,
    prelude::*,
    render::{
        settings::{Backends, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    DefaultPlugins,
};

mod fly_by_cam;
mod map;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::VULKAN),
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            },
        }))
        .add_plugins((
            fly_by_cam::FlyByCameraPlugin,
            map::MapPlugin,
            // player::PlayerPlugin,
        ))
        .add_systems(Update, (hold_esc_to_exit, toggle_camera))
        .add_systems(Startup, setup_camera)
        .run();
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

fn toggle_camera(
    mut cam_config: ResMut<fly_by_cam::FlyByCameraConfig>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_released(KeyCode::Escape) {
        cam_config.active = !cam_config.active;
    }
}

fn hold_esc_to_exit(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut accum_hold: Local<f32>,
    mut exit_evt_writer: EventWriter<AppExit>,
) {
    if input.pressed(KeyCode::Escape) {
        *accum_hold += time.delta_seconds();
    } else {
        *accum_hold = 0.0;
    }

    if *accum_hold >= 0.5 {
        exit_evt_writer.send(AppExit);
    }
}
