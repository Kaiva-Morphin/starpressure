use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod rooms;

use rooms::*;
use crate::appstates::AppState;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            process_air_leak, apply_air_force, paint_walls)
        .run_if(in_state(AppState::InGame)))
        ;
    }
}
