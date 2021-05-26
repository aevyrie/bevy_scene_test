use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::AmbientLight,
    prelude::*,
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("models/sponza/Sponza.gltf#Scene0"));
    commands.spawn_scene(asset_server.load("models/monkey/Monkey.gltf#Scene0"));
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0., 0., 20.)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera::default());
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 5.0, 3.0),
        ..Default::default()
    });
}
