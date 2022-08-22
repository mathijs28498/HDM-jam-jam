use bevy::prelude::*;

use super::components::Monster;

pub(crate) fn move_ally_creature_system(mut creature_query: Query<&mut Transform, With<Monster>>) {
    for mut transform in creature_query.iter_mut() {
        transform.translation.x -= 1.;
    }
}
