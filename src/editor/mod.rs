use bevy::prelude::*;

use crate::appstates::GameState;

pub mod systems;
mod components;
mod interactions;
mod ui;
mod atlas;

use systems::*;
use interactions::*;
use ui::*;
use atlas::*;

use self::components::{FileOpenWindowEvent, LoadAtlasEvent, LoadedAtlas, NewFileEvent, OpenFileEvent, ResizeEvent, SaveFileEvent, UiVars};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ResizeEvent>()
        .add_event::<FileOpenWindowEvent>()
        .add_event::<NewFileEvent>()
        .add_event::<SaveFileEvent>()
        .add_event::<OpenFileEvent>()
        .add_event::<LoadAtlasEvent>()
        .insert_resource(UiVars::default())
        .insert_resource(LoadedAtlas {handle: Handle::default()})
        .add_systems(OnEnter(GameState::Editor), init_file_button)
        .add_systems(Update, (
            update_tab, manage_file_window, new_file, interact_file, interact_new_file_tab, interact_open_file_tab,
            interact_save_file_tab, dialog, save_open_file, interact_load_atlas_tab, update_atlas, load_atlas, 
        ).run_if(in_state(GameState::Editor)))
        ;
    }
}
