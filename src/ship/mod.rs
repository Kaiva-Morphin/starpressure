use bevy::prelude::*;

mod tiles;

use tiles::*;
use crate::appstates::AppState;

use self::systems::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::InGame), (init_room, spawn_camera, spawn_box))
        .add_systems(Update, (process_air_leak, apply_air_force, paint_walls)
        .run_if(in_state(AppState::InGame)))
        ;
    }
}
