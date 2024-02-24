use bevy::prelude::*;

mod tiles;
mod constructor;
mod components;
mod systems;

use tiles::*;
use crate::appstates::AppState;

use self::{constructor::systems::*, systems::*, tiles::systems::*};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::InGame), (spawn_camera, spawn_box))
        .add_systems(Update, (process_air_leak, apply_air_force, paint_walls, save_ship, load_ship)
        .run_if(in_state(AppState::InGame)))
        ;
    }
}
