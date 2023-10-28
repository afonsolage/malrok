use bevy::{
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
        .add_systems(Update, bevy::window::close_on_esc)
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
