use bevy::{prelude::*, utils::HashSet};
use bevy_file_dialog::prelude::*;
use bincode;
use serde::Serialize;

use crate::{components::{CursorPosition, WindowSize}, ragdoll::{components::{RagdollSave, SelectionSave}, systems::load_atlas}, AtlasFileContents, RagdollFileContents};

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
    mut atlas_data: ResMut<AtlasData>,
    asset_server: Res<Assets<Image>>,
) {
    // open
    for _ in open_file_event.read() {
        commands
        .dialog()
        .add_filter("Ragdoll Binary", &["bin"])
        .load_file::<RagdollFileContents>();
    }
    // save
    // todo: there must be only one selection with no parent and no loops in selections graph, impl check for this
    for _ in save_file_event.read() {
        let mut save = RagdollSave {
            name: atlas_data.name.clone(),
            saves: vec![],
        };
        let sorted_selections = sort_selections(&atlas_data.selections.clone().into_values().collect());
        for selection in sorted_selections {
            save.saves.push(SelectionSave::new(
                selection.ulrect,
                selection.lpos,
                selection.entity,
                selection.joints.clone(),
                selection.parents.clone(),
            ))
        }
        commands
        .dialog()
        .add_filter("Ragdoll Binary", &["bin"])
        .set_file_name("ragdoll.bin")
        .save_file::<RagdollFileContents>(bincode::serialize(&save).unwrap());
    }
    // load
    for _ in load_atlas_event.read() {
        commands
        .dialog()
        .add_filter("Texture Atlas", &["png"])
        .load_file::<AtlasFileContents>();
    }
}

fn sort_selections(
    selections: &Vec<Selection>,
) -> Vec<Selection> {
    let mut order = vec![];
    for (id, selection) in selections.into_iter().enumerate() {
        if selection.parents.is_empty() {
            order.push(id);
            break;
        }
    }
    let mut visited = HashSet::new();
    let mut starters = vec![order[0]];
    loop {
        let mut new_starters = vec![];
        for starter in starters {
            for (child, _joint) in selections[starter].joints.iter() {
                if !visited.contains(&child) {
                    let order_id = selections.iter().position(|x| &x.entity == child).unwrap();
                    visited.insert(child);
                    new_starters.push(order_id);
                    order.push(order_id);   
                }
            }
        }
        if new_starters.is_empty() {
            break;
        }
        starters = new_starters;
    }
    let mut new = Vec::new();
    for _ in 0..selections.len() { new.push(Selection::empty()) }
    for (index, order_id) in order.into_iter().enumerate() {
        new[index] = selections[order_id].clone();
    }
    new
}