use bevy::prelude::*;
use bevy_proto::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
pub(crate) struct Stats {
    health: i32,
    damage: i32,
}

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
pub(crate) struct Enemy;

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
pub(crate) struct Monster;

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
pub(crate) struct Warrior;

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
pub(crate) struct Target;

/// Common for both buff and debuff
pub(crate) enum Buff {
    Shock, Slow, Regen,
}
