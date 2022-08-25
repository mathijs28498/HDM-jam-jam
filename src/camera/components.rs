use bevy::prelude::*;

// Used to determine which way to move the camera when hovering the sides with your mouse
#[derive(Component, Debug)]
pub enum CameraMoveBoxDirection {
    Left(bool),
    Right(bool),
}

// Configuration for the movement of the camera
#[derive(Component, Debug)]
pub(crate) struct CameraMovementConfig {
    pub(crate) speed: f32,
    pub(crate) min_x: f32,
    pub(crate) max_x: f32,
}
