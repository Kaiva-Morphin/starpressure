use bevy::prelude::*;
use bevy_file_dialog::prelude::*;
use bincode;

use crate::{components::{CursorPosition, WindowSize}, AtlasFileContents, RagdollFileContents};

use super::{components::*, HOVER_COLOR, SECONDARY_COLOR, TEXT_COLOR};

pub fn interact_file(
    mut button_q: Query<(&Interaction, &mut BackgroundColor, Entity, &mut FileButton), Changed<Interaction>>,
    mut file_event: EventWriter<FileOpenWindowEvent>,
) {
    if let Ok((interaction, mut backgroundcolor, entity, mut button))
    = button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = HOVER_COLOR.into();
                if button.is_opened {
                    file_event.send(FileOpenWindowEvent { entity, to_open: false });
                } else {
                    file_event.send(FileOpenWindowEvent { entity, to_open: true });
                }
                button.is_opened = !button.is_opened;
            }
            Interaction::Hovered => {
                *backgroundcolor = HOVER_COLOR.into();
            }
            Interaction::None => {
                *backgroundcolor = SECONDARY_COLOR.into();
            }
        }
    }
}

pub fn interact_new_file_tab( // todo: optim every interact query
    mut new_file_button_q: Query<(&Interaction, &mut BackgroundColor, Entity, &mut NewFileButton), Changed<Interaction>>,
    mut new_file_event: EventWriter<NewFileEvent>,
) {
    if let Ok((interaction, mut backgroundcolor, _, _))
    = new_file_button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = HOVER_COLOR.into();
                new_file_event.send(NewFileEvent {});
            }
            Interaction::Hovered => {
                *backgroundcolor = HOVER_COLOR.into();
            }
            Interaction::None => {
                *backgroundcolor = SECONDARY_COLOR.into();
            }
        }
    }
}

pub fn interact_open_file_tab(
    mut open_file_button_q: Query<(&Interaction, &mut BackgroundColor, Entity, &mut OpenFileButton), Changed<Interaction>>,
    mut open_file_event: EventWriter<OpenFileEvent>,
) {
    if let Ok((interaction, mut backgroundcolor, _, _))
    = open_file_button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = HOVER_COLOR.into();
                open_file_event.send(OpenFileEvent {});
            }
            Interaction::Hovered => {
                *backgroundcolor = HOVER_COLOR.into();
            }
            Interaction::None => {
                *backgroundcolor = SECONDARY_COLOR.into();
            }
        }
    }
}

pub fn interact_save_file_tab(
    mut save_file_button_q: Query<(&Interaction, &mut BackgroundColor, Entity, &mut SaveFileButton), Changed<Interaction>>,
    mut save_file_event: EventWriter<SaveFileEvent>,
) {
    if let Ok((interaction, mut backgroundcolor, _, _))
    = save_file_button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = HOVER_COLOR.into();
                save_file_event.send(SaveFileEvent {});
            }
            Interaction::Hovered => {
                *backgroundcolor = HOVER_COLOR.into();
            }
            Interaction::None => {
                *backgroundcolor = SECONDARY_COLOR.into();
            }
        }
    }
}

pub fn interact_load_atlas_tab(
    mut save_file_button_q: Query<(&Interaction, &mut BackgroundColor, Entity, &mut LoadAtlasButton), Changed<Interaction>>,
    mut load_atlas_event: EventWriter<LoadAtlasEvent>,
) {
    if let Ok((interaction, mut backgroundcolor, _, _))
    = save_file_button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = HOVER_COLOR.into();
                load_atlas_event.send(LoadAtlasEvent {});
            }
            Interaction::Hovered => {
                *backgroundcolor = HOVER_COLOR.into();
            }
            Interaction::None => {
                *backgroundcolor = SECONDARY_COLOR.into();
            }
        }
    }
}

pub fn dialog(
    mut commands: Commands,
    mut open_file_event: EventReader<OpenFileEvent>,
    mut save_file_event: EventReader<SaveFileEvent>,
    mut load_atlas_event: EventReader<LoadAtlasEvent>,
) {
    // open
    for _ in open_file_event.read() {
        commands
        .dialog()
        .add_filter("Ragdoll Binary", &["bin"])
        .load_file::<RagdollFileContents>();
    }
    // save
    for _ in save_file_event.read() {
        commands
        .dialog()
        .add_filter("Ragdoll Binary", &["bin"])
        .set_file_name("ragdoll.bin")
        .save_file::<RagdollFileContents>(b"hello".to_vec());
    }
    // load
    for _ in load_atlas_event.read() {
        commands
        .dialog()
        .add_filter("Texture Atlas", &["png"])
        .load_file::<AtlasFileContents>();
    }
}

pub fn save_open_file(
    mut open_event: EventReader<DialogFileLoaded<RagdollFileContents>>,
    mut save_event: EventReader<DialogFileSaved<RagdollFileContents>>,
) {
    for open_event in open_event.read() {
        println!("{:?}", open_event.path);
    }
    for save_event in save_event.read() {

    }
}