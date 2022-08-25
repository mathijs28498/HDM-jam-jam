use bevy::prelude::*;

use crate::NORMAL_BUTTON;

use self::components::{SpawnButton,PartPosition, SwapButtonPosition};
use crate::{BaseStorage,GameAssets};

pub(crate) mod components;
pub(crate) mod systems;

pub(crate) fn setup_creation_ui(mut commands: Commands, assets: Res<GameAssets>, bs: Res<BaseStorage>) {
    commands
        .spawn_bundle(NodeBundle {
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
            
                flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        color: Color::rgb(0.7, 0.7, 0.7).into(),
        ..default()
    })
    .with_children(|parent| {
            let child_bundle = NodeBundle {
            style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                ..default()
            },
                color: Color::NONE.into(),
            ..default()
            };

            bs.spawn_board(parent,&assets);

            // Head swap node
            spawn_monster_segment_node(
                parent,
                child_bundle.clone(),
                Color::rgb(1., 0., 0.).into(),
                &assets,
                PartPosition::Head,
            );

            // Body swap node
            spawn_monster_segment_node(
                parent,
                child_bundle.clone(),
                Color::rgb(0., 1., 0.).into(),
                &assets,
                PartPosition::Body,
            );

            // Legs swap node
            spawn_monster_segment_node(
                parent,
                child_bundle,
                Color::rgb(0., 1., 1.).into(),
                &assets,
                PartPosition::Legs,
            );

            // Spawn button
            parent
                .spawn_bundle(ButtonBundle {
            style: Style {
                        size: Size::new(Val::Px(200.), Val::Px(40.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            top: Val::Px(30.),
                ..default()
            },
            ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(assets.create_text("Spawn"));
                })
                .insert(SpawnButton);
        });
}

fn spawn_monster_segment_node(
    mut parent: &mut ChildBuilder,
    child_bundle: NodeBundle,
    color: UiColor,
    assets: &GameAssets,
    part: PartPosition,
) {
    parent.spawn_bundle(child_bundle).with_children(|parent| {
        parent
            .spawn_bundle(assets.button())
            .with_children(|parent| {
                parent.spawn_bundle(assets.create_text("<"));
    });

        parent.spawn_bundle(create_default_image_bundle(color));

        parent
            .spawn_bundle(assets.button())
            .with_children(|parent| {
                parent.spawn_bundle(assets.create_text(">"));
            });
    }).insert(part);
}


fn create_default_image_bundle(color: UiColor) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Px(100.), Val::Px(100.)),
            margin: UiRect {
                left: Val::Px(25.),
                right: Val::Px(25.),
                ..default()
                },
            ..default()
        },
        color,
        ..default()
    }
}
