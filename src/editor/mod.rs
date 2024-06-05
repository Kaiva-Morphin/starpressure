use bevy::prelude::*;

use crate::appstates::GameState;

pub mod systems;
pub mod components;
pub mod interactions;
mod ui;
mod atlas;

use systems::*;
use interactions::*;
use ui::*;
use atlas::*;

use self::components::{
    AtlasData, CursorAboveUi, FileOpenWindowEvent, JointSelectionOver, LoadAtlasEvent, NewFileEvent, OpenFileEvent, ResizeEvent, SaveFileEvent};

pub struct EditorPlugin;
// todo: ON EXIT DELETE ALL RESOURCES!!!
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ResizeEvent>()
        .add_event::<FileOpenWindowEvent>()
        .add_event::<NewFileEvent>()
        .add_event::<SaveFileEvent>()
        .add_event::<OpenFileEvent>()
        .add_event::<LoadAtlasEvent>()
        .add_event::<JointSelectionOver>()
        .add_event::<CursorAboveUi>()
        .init_state::<JointSelectionState>()
        .add_systems(OnEnter(GameState::Editor), (load_resources, init_file_button))
        .add_systems(Update, (
            manage_file_window, new_file, interact_file, interact_new_file_tab, interact_open_file_tab,
            interact_save_file_tab, dialog, save_open_file, interact_load_atlas_tab, load_atlas, atlas_controller,
            update_node_controller, 
        ).run_if(in_state(GameState::Editor)))
        .add_systems(OnExit(GameState::Editor), unload_resources)
        ;
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum JointSelectionState {
    #[default]
    N,
    Y,
}
