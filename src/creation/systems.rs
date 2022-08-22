use bevy::prelude::*;
use bevy_proto::prelude::ProtoData;

use crate::{combat::components::Monster, HOVERED_BUTTON, NORMAL_BUTTON, PIXEL, PRESSED_BUTTON};

use super::{
    components::{PartPosition, PartType, PartTypeList, SpawnButton, SwapButtonPosition},
    spawn_button,
};

pub(crate) fn interaction_spawn_button_system(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    prototypes: Res<ProtoData>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let monster = prototypes
            .get_prototype("Monster")
            .expect("Could not get Monster prototype");

        monster
            .spawn(&mut commands, &prototypes, &asset_server)
            .insert_bundle(SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    scale: Vec3::new(4., 4., 1.),
                    ..default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("images/sprite.png"),
                    transform: Transform {
                        translation: Vec3::new(0., 0., 1.),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    ..default()
                });

                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("images/sprite.png"),
                    transform: Transform {
                        translation: Vec3::new(0., -PIXEL, 1.),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..default()
                    },
                    ..default()
                });

                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("images/sprite.png"),
                    transform: Transform {
                        translation: Vec3::new(0., -(PIXEL * 2.), 1.),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::BLUE,
                        ..default()
                    },
                    ..default()
                });
            });
    }
}

pub(crate) fn interaction_swap_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &SwapButtonPosition),
        (Changed<Interaction>, With<Button>),
    >,
    part_type_list: Res<PartTypeList>,
    mut part_query: Query<(&mut PartType, &mut Sprite, &PartPosition)>,
) {
    for (interaction, mut color, swap_button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match swap_button_type {
                    SwapButtonPosition::Head(is_right) => {
                        for (mut part_type, mut sprite, part_position) in part_query.iter_mut() {
                            match part_position {
                                PartPosition::Head => {}
                                _ => continue,
                            }
                            let old_index = part_type_list
                                .part_type_list
                                .iter()
                                .position(|pt| pt.color == part_type.color)
                                .unwrap();
                            let addition = if *is_right { 1 } else { -1 };
                            let new_index = (old_index as i32 + addition)
                                .rem_euclid(part_type_list.part_type_list.len() as i32)
                                as usize;

                            *part_type = part_type_list.part_type_list[new_index].clone();
                            sprite.color = part_type.color;

                            break;
                        }
                    }
                    SwapButtonPosition::Body(is_right) => {
                        for (mut part_type, mut sprite, part_position) in part_query.iter_mut() {
                            match part_position {
                                PartPosition::Body => {}
                                _ => continue,
                            }
                            let old_index = part_type_list
                                .part_type_list
                                .iter()
                                .position(|pt| pt.color == part_type.color)
                                .unwrap();
                            let addition = if *is_right { 1 } else { -1 };
                            let new_index = (old_index as i32 + addition)
                                .rem_euclid(part_type_list.part_type_list.len() as i32)
                                as usize;

                            *part_type = part_type_list.part_type_list[new_index].clone();
                            sprite.color = part_type.color;

                            break;
                        }
                    }
                    SwapButtonPosition::Legs(is_right) => {
                        for (mut part_type, mut sprite, part_position) in part_query.iter_mut() {
                            match part_position {
                                PartPosition::Legs => {}
                                _ => continue,
                            }
                            let old_index = part_type_list
                                .part_type_list
                                .iter()
                                .position(|pt| pt.color == part_type.color)
                                .unwrap();
                            let addition = if *is_right { 1 } else { -1 };
                            let new_index = (old_index as i32 + addition)
                                .rem_euclid(part_type_list.part_type_list.len() as i32)
                                as usize;

                            *part_type = part_type_list.part_type_list[new_index].clone();
                            sprite.color = part_type.color;

                            break;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub(crate) fn spawn_swap_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    spawn_button(
        commands,
        asset_server,
        UiRect {
            right: Val::Px(240.),
            top: Val::Px(220.),
            ..default()
        },
        "<",
        SwapButtonPosition::Head(false),
    );

    spawn_button(
        commands,
        asset_server,
        UiRect {
            right: Val::Px(40.),
            top: Val::Px(220.),
            ..default()
        },
        ">",
        SwapButtonPosition::Head(true),
    );

    spawn_button(
        commands,
        asset_server,
        UiRect {
            right: Val::Px(240.),
            top: Val::Px(340.),
            ..default()
        },
        "<",
        SwapButtonPosition::Body(false),
    );

    spawn_button(
        commands,
        asset_server,
        UiRect {
            right: Val::Px(40.),
            top: Val::Px(340.),
            ..default()
        },
        ">",
        SwapButtonPosition::Body(true),
    );

    spawn_button(
        commands,
        asset_server,
        UiRect {
            right: Val::Px(240.),
            top: Val::Px(460.),
            ..default()
        },
        "<",
        SwapButtonPosition::Legs(false),
    );

    spawn_button(
        commands,
        asset_server,
        UiRect {
            right: Val::Px(40.),
            top: Val::Px(460.),
            ..default()
        },
        ">",
        SwapButtonPosition::Legs(true),
    );
}
