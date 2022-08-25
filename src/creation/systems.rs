use bevy::prelude::*;
use bevy_proto::prelude::ProtoData;

use crate::{
    combat::components::Monster, BaseStorage, GameAssets, HOVERED_BUTTON, NORMAL_BUTTON, PIXEL,
    PRESSED_BUTTON,
};

use super::components::{
    Element, PartPosition, PartType, PartTypeList, SpawnButton, SwapButtonPosition,
};

pub(crate) fn interaction_spawn_button_system(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    prototypes: Res<ProtoData>,
    mut bs: ResMut<BaseStorage>,
    assets: Res<GameAssets>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let monster = prototypes
            .get_prototype("Monster")
            .expect("Could not get Monster prototype");

        if bs.can_consume_escrow() {
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
                parent.spawn_bundle(assets.spawn_elem(bs.head.0,0.));
                parent.spawn_bundle(assets.spawn_elem(bs.body.0,1.));
                parent.spawn_bundle(assets.spawn_elem(bs.legs.0,2.));
            });
            bs.consume_escrow();
        }
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

pub(crate) fn work_shop(
    parts: Query<(&Children, &PartPosition)>,
    buttons: Query<&Interaction, Changed<Interaction>>,
    assets: Res<GameAssets>,
    mut colors: Query<&mut UiColor>,
    mut bs: ResMut<BaseStorage>,
){
    for (children, part) in parts.iter() {
        let mut escrow = match part {
            PartPosition::Head => &mut bs.head.0,
            PartPosition::Body => &mut bs.body.0,
            PartPosition::Legs => &mut bs.legs.0,
        };
        let mut color = colors.get_mut(children[1]).unwrap();
        color.0 = assets.elem_sprite_material(*escrow).color;
        if let Some(change) = children.iter()
            .position(|&x| buttons.get(x) == Ok(&Interaction::Clicked)){
                *escrow += change+3;
                *escrow %= 4;
            }
    }

}
