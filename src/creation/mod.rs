use bevy::prelude::*;

use crate::{NORMAL_BUTTON, WIDTH, HEIGHT};

use self::{components::SwapButtonPosition, systems::spawn_swap_buttons};

pub(crate) mod components;
pub(crate) mod systems;

pub(crate) fn setup_creation_ui(mut commands: Commands) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(25.), Val::Percent(100.)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(0.),
                top: Val::Px(0.),
                ..default()
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            
            flex_direction: FlexDirection::Column,
            ..default()
        },
        color: Color::rgb(0.7, 0.7, 0.7).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(20.), Val::Px(20.)),
                ..default()
            },
            color: Color::rgb(0., 0., 0.).into(),
            ..default()
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(20.), Val::Px(20.)),
                ..default()
            },
            color: Color::rgb(0., 0., 0.).into(),
            ..default()
        });
        // parent.spawn_bundle(NodeBundle {
        //     style: Style {
        //         size: Size::new(Val::Px(20.), Val::Px(20.)),
        //         ..default()
        //     },
        //     color: Color::rgb(0., 0., 0.).into(),
        //     ..default()
        // });
    });
    // spawn_swap_buttons(&mut commands, &asset_server);

    // let head_part_type = part_type_list.part_type_list[0].clone();
    // let body_part_type = part_type_list.part_type_list[1].clone();
    // let legs_part_type = part_type_list.part_type_list[2].clone();

    // commands
    //     .spawn_bundle(ButtonBundle {
    //         style: Style {
    //             size: Size::new(Val::Px(200.), Val::Px(40.)),
    //             position_type: PositionType::Absolute,
    //             position: UiRect {
    //                 right: Val::Px(60.),
    //                 bottom: Val::Px(100.),
    //                 ..default()
    //             },
    //             // Make text align in center
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             ..default()
    //         },
    //         color: NORMAL_BUTTON.into(),
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn_bundle(TextBundle::from_section(
    //             "Spawn",
    //             TextStyle {
    //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                 font_size: 40.0,
    //                 color: Color::rgb(0.9, 0.9, 0.9),
    //             },
    //         ));
    //     })
    //     .insert(SpawnButton);
}

pub(crate) fn spawn_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: UiRect<Val>,
    text: &str,
    swap_button_type: SwapButtonPosition,
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(40.), Val::Px(40.)),
                position_type: PositionType::Absolute,
                position,
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
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(swap_button_type);
}
