mod combat;
mod creation;

use bevy::prelude::*;
use combat::systems::move_ally_creature_system;
use creation::{
    components::{PartPosition, PartType, PartTypeList, SpawnButton},
    systems::{spawn_button_system, spawn_swap_buttons, swap_button_system},
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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
        .add_system(swap_button_system)
        .add_system(spawn_button_system)
        .add_system(move_ally_creature_system)
        .run();
}

fn setup(
    mut commands: Commands,
    part_type_list: Res<PartTypeList>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/light_blue.png"),
        transform: Transform {
            translation: Vec3::new(-160., 100., 1.),
            scale: Vec3::new(960., 520., 1.),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/green.png"),
        transform: Transform {
            translation: Vec3::new(-160., -260., 1.),
            scale: Vec3::new(960., 200., 1.),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/gray.png"),
        transform: Transform {
            translation: Vec3::new(480., 0., 1.),
            scale: Vec3::new(320., 720., 1.),
            ..default()
        },
        ..default()
    });

    spawn_swap_buttons(&mut commands, &asset_server);

    let head_part_type = part_type_list.part_type_list[0].clone();
    let body_part_type = part_type_list.part_type_list[1].clone();
    let legs_part_type = part_type_list.part_type_list[2].clone();

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.), Val::Px(40.)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(60.),
                    bottom: Val::Px(100.),
                    ..default()
                },
                // Make text align in center
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Spawn",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(SpawnButton);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/white.png"),
            transform: Transform {
                translation: Vec3::new(480., 120., 2.),
                scale: Vec3::new(120., 120., 1.),
                ..default()
            },
            sprite: Sprite {
                color: head_part_type.color,
                ..default()
            },
            ..default()
        })
        .insert(PartPosition::Head)
        .insert(head_part_type);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/white.png"),
            transform: Transform {
                translation: Vec3::new(480., 0., 2.),
                scale: Vec3::new(120., 120., 1.),
                ..default()
            },
            sprite: Sprite {
                color: body_part_type.color,
                ..default()
            },
            ..default()
        })
        .insert(PartPosition::Body)
        .insert(body_part_type);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/white.png"),
            transform: Transform {
                translation: Vec3::new(480., -120., 2.),
                scale: Vec3::new(120., 120., 1.),
                ..default()
            },
            sprite: Sprite {
                color: legs_part_type.color,
                ..default()
            },
            ..default()
        })
        .insert(PartPosition::Legs)
        .insert(legs_part_type);
}
