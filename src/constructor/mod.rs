use bevy::prelude::*;

use crate::{
    appstates::GameState,
    editor::interactions::{
        interact_file, interact_new_file_tab, interact_open_file_tab, interact_save_file_tab
    }
};

pub mod systems;
pub mod components;
pub mod ui;

use ui::*;

use self::{components::DrawBlueprint, systems::*};

pub struct ConstructorPlugin;

impl Plugin for ConstructorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Constructor), init_file_button)
        .add_event::<DrawBlueprint>()
        .init_state::<ConstructorState>()
        .add_systems(Update, 
            (manage_file_window, interact_file, interact_new_file_tab, interact_open_file_tab,
                interact_save_file_tab, dialog, new_file, process_selection, draw_blueprint,
            ).run_if(in_state(GameState::Constructor)))
        ;
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ConstructorState {
    #[default]
    Walls,
    Tiles,
}
