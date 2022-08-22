use bevy::prelude::*;

use crate::NORMAL_BUTTON;

use self::components::{SpawnButton, SwapButtonPosition};

pub(crate) mod components;
pub(crate) mod systems;

pub(crate) fn setup_creation_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

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

            // Head swap node
            spawn_monster_segment_node(
                parent,
                child_bundle.clone(),
                Color::rgb(1., 0., 0.).into(),
                &font,
            );

            // Body swap node
            spawn_monster_segment_node(
                parent,
                child_bundle.clone(),
                Color::rgb(0., 1., 0.).into(),
                &font,
            );

            // Legs swap node
            spawn_monster_segment_node(
                parent,
                child_bundle.clone(),
                Color::rgb(0., 1., 1.).into(),
                &font,
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
                    parent.spawn_bundle(create_default_text_bundle("Spawn", font.clone()));
                })
                .insert(SpawnButton);
        });
}

fn spawn_monster_segment_node(
    mut parent: &mut ChildBuilder,
    child_bundle: NodeBundle,
    color: UiColor,
    font: &Handle<Font>,
) {
    parent.spawn_bundle(child_bundle).with_children(|parent| {
        parent
            .spawn_bundle(create_default_button_bundle())
            .with_children(|parent| {
                parent.spawn_bundle(create_default_text_bundle("<", font.clone()));
            });

        parent.spawn_bundle(create_default_image_bundle(color));

        parent
            .spawn_bundle(create_default_button_bundle())
            .with_children(|parent| {
                parent.spawn_bundle(create_default_text_bundle(">", font.clone()));
            });
    });
}

fn create_default_text_bundle(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    )
}

fn create_default_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(40.), Val::Px(40.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    }
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
