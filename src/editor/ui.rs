use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::WindowSize;

use super::components::{AtlasData, Tile};

pub fn update_node_controller(
    mut commands: Commands,
    mut egui_context: EguiContexts,
    mut vars: ResMut<AtlasData>,
    window_size: Res<WindowSize>,
    mut tile_q: Query<(&mut Tile, &mut Transform)>
) {
    if let Some(selected_id) = vars.selected {
        let texture = vars.image.clone();
        let scale = vars.scale.clone();
        let max = vars.rect.max.x;
        let may = vars.rect.max.y;
        let (selected_rect, selected_pos, selected_entity) = &mut vars.selections[selected_id];
        //let mut selected_rect = selected_rect.clone();
        let ctx = egui_context.ctx_mut();
        let (mut tile, mut tile_transform) = tile_q.get_mut(*selected_entity).unwrap();
        let translation = &mut tile_transform.translation;
        let mut changed = false;
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
            
            ui.label("Tile position on atlas x");
            let res = ui.add(egui::DragValue::new(&mut selected_pos.x).speed(0.06));
            if res.changed() {
                changed = true;
            }
            
            ui.label("Tile position on atlas y");
            let res = ui.add(egui::DragValue::new(&mut selected_pos.y).speed(0.06));
            if res.changed() {
                changed = true;
            }
            
            ui.label("Width");
            let mut width = selected_rect.max.x;
            let res = ui.add(egui::DragValue::new(&mut width).speed(0.06));
            if res.changed() {
                changed = true;
                width = width.clamp(selected_rect.min.x, max);
                selected_pos.x += (width - selected_rect.max.x) / 2.;
                selected_rect.max.x = width;
            }

            ui.label("Height");
            let mut height = selected_rect.max.y;
            let res = ui.add(egui::DragValue::new(&mut height).speed(0.06));
            if res.changed() {
                changed = true;
                height = height.clamp(selected_rect.min.y, may);
                selected_pos.y += (height - selected_rect.max.y) / 2.;
                selected_rect.max.y = height;
            }
        });
        if changed {
            println!("{:?}", selected_rect);
            let mut luregion = selected_rect.clone();
            luregion.min /= scale;
            luregion.max /= scale;
            luregion = Rect::from_center_size(
                *selected_pos,
                Vec2::new(luregion.max.x - luregion.min.x, luregion.max.y - luregion.min.y)
            );
            println!("{:?}", luregion);
            commands.entity(*selected_entity).insert(
                SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(0.,0.,0.,),
                    sprite: Sprite {
                        rect: Some(luregion),
                        ..default()
                    },
                    ..default()
                },
            );
        }
    }
}