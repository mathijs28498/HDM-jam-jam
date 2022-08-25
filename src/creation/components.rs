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
pub enum Element {
    Mud, Water, Fire, Electric
}
impl From<usize> for Element {
    fn from(t: usize) -> Self {
        match t {
            0 => Self::Mud,
            1 => Self::Water,
            2 => Self::Fire,
            3 => Self::Electric,
            _ => unreachable!(),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub(crate) struct PartType {
    pub(crate) color: Color,
}

#[derive(Component)]
pub(crate) struct PartTypeList {
    pub(crate) part_type_list: Vec<PartType>,
}
