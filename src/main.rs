use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/elemental_test_image.png"),
        // transform: Transform {
        //     translation: Vec3::new(10., 10., 1.),
        //     ..default()
        // },
        ..default()
    });
}