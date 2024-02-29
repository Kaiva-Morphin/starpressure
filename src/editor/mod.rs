use bevy::prelude::*;

use crate::appstates::GameState;

pub mod systems;
mod components;
mod interactions;

use systems::*;
use interactions::*;

use self::components::{FileOpenWindowEvent, NewFileEvent, OpenFileEvent, ResizeEvent, SaveFileEvent};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ResizeEvent>()
        .add_event::<FileOpenWindowEvent>()
        .add_event::<NewFileEvent>()
        .add_event::<SaveFileEvent>()
        .add_event::<OpenFileEvent>()
        .add_systems(OnEnter(GameState::Editor), init_editor_ui)
        .add_systems(Update, (
            interact_resizer, update_editor_ui, interact_file, manage_file_window, dialog,
            save_load_file, interact_new_file_tab, interact_save_file_tab, interact_open_file_tab
        ).run_if(in_state(GameState::Editor)))
        ;
    }
}
