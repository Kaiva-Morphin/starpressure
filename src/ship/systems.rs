use bevy::prelude::*;

use super::components::{PlayerShip, Ship};

pub fn test(
    ship_q: Query<Entity, With<PlayerShip>>,
    world: &World,
) {
    let entity = ship_q.single();
    let entity_ref = world.get_entity(entity).unwrap();
    entity_ref.get::<Transform>().unwrap();
}