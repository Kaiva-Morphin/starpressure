use bevy::prelude::*;

use crate::appstates::AppState;

pub mod systems;
pub mod components;

use systems::*;
use components::*;

pub struct RagdollPlugin;

impl Plugin for RagdollPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Name2Handle::new())
        .add_event::<RagdollSave>()
        .add_systems(Update, (load_ragdoll, ph))
        ;
    }
}