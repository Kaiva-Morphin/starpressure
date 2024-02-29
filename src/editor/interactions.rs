use bevy::prelude::*;
use bevy_file_dialog::prelude::*;
use bincode;

use crate::{components::{CursorPosition, WindowSize}, RagdollFileContents};

use super::{components::*, HOVER_COLOR, SECONDARY_COLOR, TEXT_COLOR};

pub fn interact_resizer(
    mut button_q: Query<(&Interaction, &mut BackgroundColor, &mut Resizer, Entity)>,
    mut resize_event: EventWriter<ResizeEvent>,
    cursor_pos: Res<CursorPosition>,
    window_size: Res<WindowSize>,
) {
    if let Ok((interaction, mut backgroundcolor, mut resizer, entity))
    = button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = Color::DARK_GRAY.into();
                resizer.was_pressed = true;
                resizer.start_pos = cursor_pos.pos.y;
                let new_pos;
                if resizer.is_vertical {
                    let t = window_size.height as f32;
                    new_pos = (t - cursor_pos.pos.y) / t * 100.;
                } else {
                    new_pos = cursor_pos.pos.x / window_size.width as f32 * 100.;
                }
                resize_event.send(ResizeEvent { new_pos, entity, is_vertical: resizer.is_vertical });
            }
            _ => {
                *backgroundcolor = Color::BLACK.into();
            }
        }
    }
}

pub fn update_editor_ui(
    mut resize_event: EventReader<ResizeEvent>,
    mut selection_tab: Query<&mut Style>,
    parents_q: Query<&Parent>,
) {
    for event in resize_event.read() {
        let parent_entity = parents_q.get(event.entity).unwrap().get();
        let mut style = selection_tab.get_mut(parent_entity).unwrap();
        if event.is_vertical {
            style.height = Val::Percent(event.new_pos);
        } else {
            style.width = Val::Percent(event.new_pos);
        }
    }
}

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

pub fn interact_new_file_tab(
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

pub fn dialog(
    mut commands: Commands,
    mut open_file_event: EventReader<OpenFileEvent>,
    mut save_file_event: EventReader<SaveFileEvent>,
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
}

pub fn save_load_file(
    mut open_event: EventReader<DialogFileLoaded<RagdollFileContents>>,
    mut save_event: EventReader<DialogFileSaved<RagdollFileContents>>,
) {
    for open_event in open_event.read() {
        println!("{:?}", open_event.path);
    }
    for save_event in save_event.read() {

    }
}