use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::WindowSize;

use super::components::{AtlasData, Tile};

pub fn update_node_controller(
    mut egui_context: EguiContexts,
    mut vars: ResMut<AtlasData>,
    window_size: Res<WindowSize>,
    mut tile_q: Query<(&mut Tile, &mut Transform)>
) {
    if let Some(selected) = vars.selected {
        let ctx = egui_context.ctx_mut();
        let (mut tile, mut tile_transform) = tile_q.get_mut(selected).unwrap();
        let translation = &mut tile_transform.translation;
        egui::Window::new(&tile.title)
        .vscroll(true)
        .resizable(true)
        .constrain(true)
        .movable(false)
        .default_pos(egui::Pos2::new(window_size.width as f32, 0.))
        .id(egui::Id::new(666))
        .show(ctx, |ui|{
            
            ui.add(egui::TextEdit::singleline(&mut tile.title).hint_text("Title"));

            if ui.button("Add Joint").clicked() {

            }
            
            ui.label("Tile position x");
            ui.add(egui::DragValue::new(&mut translation.x).speed(0.06));
            
            ui.label("Tile position y");
            ui.add(egui::DragValue::new(&mut translation.y).speed(0.06));
            
        });
    }
}