use bevy::prelude::*;

use crate::editor::systems::{HOVER_COLOR, SECONDARY_COLOR};

use super::components::*;

pub fn interact_selection_menu(
    mut square_button_q: Query<(&Interaction, &mut BackgroundColor, &SelectionSquare), Changed<Interaction>>,
    mut selected: ResMut<SelectedTile>,
) {
    for (interaction, mut backgroundcolor, square) in square_button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = HOVER_COLOR.into();
                selected.hadle = Some(square.handle.clone());
                selected.rect = square.rect.clone();
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

pub fn interact_walls_button(
    mut square_button_q: Query<(&Interaction, &mut BackgroundColor, &WallsButton), Changed<Interaction>>,
    mut tilesorwalls: ResMut<TilesOrWalls>,
) {
    for (interaction, mut backgroundcolor, _) in square_button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = HOVER_COLOR.into();
                tilesorwalls.is_tiles = false;
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

pub fn interact_tiles_button(
    mut square_button_q: Query<(&Interaction, &mut BackgroundColor, &TilesButton), Changed<Interaction>>,
    mut tilesorwalls: ResMut<TilesOrWalls>,
) {
    for (interaction, mut backgroundcolor, _) in square_button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroundcolor = HOVER_COLOR.into();
                tilesorwalls.is_tiles = true;
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

pub fn shortcuts(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedTile>,
) {
    if keyboard_input.just_released(KeyCode::KeyQ) {
        selected.hadle = None
    }
}