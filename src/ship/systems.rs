use bevy::prelude::*;

use super::components::Ship;

pub fn spawn_ship(
    mut commands: Commands
) {
    commands.spawn(Ship)
    
    ;
}