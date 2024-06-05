use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::{
    appstates::GameState,
    editor::interactions::{
        interact_file, interact_new_file_tab, interact_open_file_tab, interact_save_file_tab
    }
};

pub mod systems;
pub mod components;
pub mod ui;
pub mod interactions;

use ui::*;

use self::{components::{DrawBlueprint}, interactions::*, systems::*};

pub struct ConstructorPlugin;

impl Plugin for ConstructorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<DrawBlueprint>()
        .add_systems(OnEnter(GameState::Constructor), (
            load_resources, init_file_button, 
        ))
        .add_systems(Update, 
            (manage_file_window, interact_file, interact_new_file_tab, interact_open_file_tab,
                interact_save_file_tab, dialog, new_file, process_selection, draw_blueprint, place, 
                interact_selection_menu, interact_walls_button, interact_tiles_button, init_selection_tab,
                shortcuts, save_ship,
            ).run_if(in_state(GameState::Constructor)))
        .add_systems(OnExit(GameState::Constructor), unload_resources)
        ;
    }
}