use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct SpawnButton;

// The type of button to swap an element
// The boolean says which button it is
// False is the left button, true is the right button
#[derive(Component)]
pub(crate) enum SwapButtonPosition {
    Head(bool),
    Body(bool),
    Legs(bool),
}

#[derive(Component)]
pub(crate) enum PartPosition {
    Head,
    Body,
    Legs,
}

#[derive(Component, Clone, Debug)]
pub(crate) struct PartType {
    pub(crate) color: Color,
}

#[derive(Component)]
pub(crate) struct PartTypeList {
    pub(crate) part_type_list: Vec<PartType>,
}
