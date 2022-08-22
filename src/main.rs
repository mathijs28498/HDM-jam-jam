mod camera;
mod combat;
mod creation;

use bevy::prelude::*;
use camera::{
    setup_camera_with_ui,
    systems::{move_camera_system, interaction_camera_move_box_system},
};
use combat::systems::move_ally_creature_system;
use creation::{
    components::{PartPosition, PartType, PartTypeList, SpawnButton},
    systems::{
        interaction_spawn_button_system, interaction_swap_button_system, spawn_swap_buttons,
    }, setup_creation_ui,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;

fn main() {
    // Window size: 1280x720
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PartTypeList {
            part_type_list: vec![
                PartType { color: Color::RED },
                PartType { color: Color::BLUE },
                PartType {
                    color: Color::GREEN,
                },
                PartType {
                    color: Color::YELLOW,
                },
            ],
        })
        .add_startup_system(setup)
        .add_startup_system(setup_camera_with_ui)
        .add_startup_system(setup_creation_ui)
        .add_system(interaction_swap_button_system)
        .add_system(interaction_spawn_button_system)
        .add_system(move_ally_creature_system)
        .add_system(move_camera_system)
        .add_system(interaction_camera_move_box_system)
        .run();
}

fn setup(
    mut commands: Commands,
    part_type_list: Res<PartTypeList>,
    asset_server: Res<AssetServer>,
) {
    let world_size = 5000.;
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/white.png"),
        transform: Transform {
            translation: Vec3::new(-160., 100., 1.),
            scale: Vec3::new(world_size, 520., 1.),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.53, 0.8, 0.9),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/white.png"),
        transform: Transform {
            translation: Vec3::new(-160., -260., 1.),
            scale: Vec3::new(world_size, 200., 1.),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.33, 0.49, 0.27),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/white.png"),
        transform: Transform {
            translation: Vec3::new(-160., 0., 1.),
            scale: Vec3::new(100., 100., 1.),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1., 0., 0.),
            ..default()
        },
        ..default()
    });

    // commands.spawn_bundle(SpriteBundle {
    //     texture: asset_server.load("images/white.png"),
    //     transform: Transform {
    //         translation: Vec3::new(WIDTH as f32 * -0.5, HEIGHT as f32 * -0.5, 1.),
    //         scale: Vec3::new(10., HEIGHT as f32, 1.),
    //         ..default()
    //     },
    //     sprite: Sprite {
    //         color: Color::rgb(1., 0., 0.),
    //         ..default()
    //     },
    //     ..default()
    // });

    

    // commands.spawn_bundle(SpriteBundle {
    //     texture: asset_server.load("images/white.png"),
    //     transform: Transform {
    //         translation: Vec3::new(-160., -260., 1.),
    //         scale: Vec3::new((WIDTH * 3 / 4) as f32, 200., 1.),
    //         ..default()
    //     },
    //     sprite: Sprite {
    //         color: Color::rgb(0.33, 0.49, 0.27),
    //         ..default()
    //     },
    //     ..default()
    // });

    // commands.spawn_bundle(SpriteBundle {
    //     texture: asset_server.load("images/white.png"),
    //     transform: Transform {
    //         translation: Vec3::new(480., 0., 1.),
    //         scale: Vec3::new(320., 720., 1.),
    //         ..default()
    //     },
    //     sprite: Sprite {
    //         color: Vec3()
    //     }
    //     ..default()
    // });


    // commands
    //     .spawn_bundle(SpriteBundle {
    //         texture: asset_server.load("images/white.png"),
    //         transform: Transform {
    //             translation: Vec3::new(480., 120., 2.),
    //             scale: Vec3::new(120., 120., 1.),
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: head_part_type.color,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(PartPosition::Head)
    //     .insert(head_part_type);

    // commands
    //     .spawn_bundle(SpriteBundle {
    //         texture: asset_server.load("images/white.png"),
    //         transform: Transform {
    //             translation: Vec3::new(480., 0., 2.),
    //             scale: Vec3::new(120., 120., 1.),
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: body_part_type.color,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(PartPosition::Body)
    //     .insert(body_part_type);

    // commands
    //     .spawn_bundle(SpriteBundle {
    //         texture: asset_server.load("images/white.png"),
    //         transform: Transform {
    //             translation: Vec3::new(480., -120., 2.),
    //             scale: Vec3::new(120., 120., 1.),
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: legs_part_type.color,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(PartPosition::Legs)
    //     .insert(legs_part_type);
}
