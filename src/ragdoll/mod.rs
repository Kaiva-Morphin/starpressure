use bevy::prelude::*;

use crate::appstates::AppState;

pub mod systems;
pub mod components;

use systems::*;

pub struct RagdollPlugin;

impl Plugin for RagdollPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_systems(OnEnter(AppState::InGame), init_skeleton)
        ;
    }
}