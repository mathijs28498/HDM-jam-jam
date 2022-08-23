use bevy::prelude::*;
use bevy_proto::prelude::*;

use crate::PIXEL;

use super::components::{Enemy, Monster};

pub(crate) fn spawn_enemy_wave(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    prototypes: Res<ProtoData>,
) {
    let warrior = prototypes
        .get_prototype("Warrior")
        .expect("Could not get Warrior prototype");

    for i in 0..5 {
        warrior
            .spawn(&mut commands, &prototypes, &asset_server)
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("images/sprite.png"),
                transform: Transform {
                    translation: Vec3::new(PIXEL * (i as f32 * 5.), 0., 1.),
                    scale: Vec3::new(4., 4., 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                ..default()
            });
    }
}

pub(crate) fn move_ally_creature_system(mut creature_query: Query<&mut Transform, With<Monster>>) {
    for mut transform in creature_query.iter_mut() {
        transform.translation.x -= 0.1;
    }
}

pub(crate) fn move_enemy_system(mut creature_query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in creature_query.iter_mut() {
        transform.translation.x += 0.1;
    }
}
