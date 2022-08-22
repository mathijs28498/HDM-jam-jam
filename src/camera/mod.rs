pub(crate) mod components;
pub(crate) mod systems;

use crate::{HEIGHT, WIDTH};
use bevy::prelude::*;

use self::components::{CameraMoveBoxDirection, CameraMovementConfig};

pub(crate) fn setup_camera_with_ui(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.insert_resource(CameraMovementConfig {
        speed: 40.,
        min_x: -500.,
        max_x: 500.
    });

    let camera_move_width_element_width = 200.;

    // Spawning UI node which is the size of the left field
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(75.), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // Spawning camera move boxes
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(camera_move_width_element_width), Val::Auto),
                        ..default()
                    },
                    color: Color::rgba(1., 0., 0., 0.1).into(),
                    ..default()
                })
                .insert(CameraMoveBoxDirection::Left(false));

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(camera_move_width_element_width), Val::Auto),
                        margin: UiRect {
                            left: Val::Auto,
                            ..default()
                        },
                        ..default()
                    },
                    color: Color::rgba(1., 0., 0., 0.1).into(),
                    ..default()
                })
                .insert(CameraMoveBoxDirection::Right(false));
        });
}
