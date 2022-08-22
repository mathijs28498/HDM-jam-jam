use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{HEIGHT, WIDTH};

use super::components::{CameraMoveBoxDirection, CameraMovementConfig};

pub(crate) fn interaction_camera_move_box_system(
    mut camera_move_box_interaction_query: Query<
        (&Interaction, &mut CameraMoveBoxDirection),
        (Changed<Interaction>, With<CameraMoveBoxDirection>),
    >,
) {
    for (interaction, mut cam_move_box_dir) in &mut camera_move_box_interaction_query {
        match *interaction {
            Interaction::Hovered => match cam_move_box_dir.as_mut() {
                CameraMoveBoxDirection::Left(do_move) => {
                    *do_move = true;
                }
                CameraMoveBoxDirection::Right(do_move) => {
                    *do_move = true;
                }
            },
            Interaction::None => match cam_move_box_dir.as_mut() {
                CameraMoveBoxDirection::Left(do_move) => {
                    *do_move = false;
                }
                CameraMoveBoxDirection::Right(do_move) => {
                    *do_move = false;
                }
            },
            _ => (),
        }
    }
}

pub(crate) fn move_camera_system(
    camera_movement_config: Res<CameraMovementConfig>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    move_box_dir_query: Query<&CameraMoveBoxDirection>,
) {
    let mut cam_transform = camera_query
        .get_single_mut()
        .expect("Failed to get the camera transform");

    for cam_move_box_dir in move_box_dir_query.iter() {
        match cam_move_box_dir {
            CameraMoveBoxDirection::Left(do_move) => {
                if *do_move {
                    cam_transform.translation.x = (cam_transform.translation.x
                        - camera_movement_config.speed)
                        .max(camera_movement_config.min_x);
                }
            }
            CameraMoveBoxDirection::Right(do_move) => {
                if *do_move {
                    cam_transform.translation.x = (cam_transform.translation.x
                        + camera_movement_config.speed)
                        .min(camera_movement_config.max_x);
                }
            }
        }
    }
}
