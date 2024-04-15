use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::WindowSize;

use super::{components::{AtlasData, CursorAboveUi, RagdollTile}, {JointSelectionState, JointSelectionOver}};

pub fn update_node_controller(
    mut commands: Commands,
    mut egui_context: EguiContexts,
    mut vars: ResMut<AtlasData>,
    window_size: Res<WindowSize>,
    mut tile_q: Query<(&mut RagdollTile, &mut Transform)>,
    mut selection_state: ResMut<NextState<JointSelectionState>>,
    mut selection_over_event: EventReader<JointSelectionOver>,
    mut cursor_above_ui: EventWriter<CursorAboveUi>
) {
    if let Some(selected_id) = vars.selected {
        let texture = vars.image.clone();
        let scale = vars.scale.clone();
        let lmax = vars.size.x;
        let lmay = vars.size.y;
        let selections = vars.selections.get_mut(&selected_id).unwrap();
        //let mut selected_rect = selected_rect.clone();
        let ctx = egui_context.ctx_mut();
        let (mut tile, mut tile_transform) = tile_q.get_mut(selections.entity).unwrap();
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

            if ui.button("Add Joint").clicked() { // todo: set as inactive if is currently used
                selection_state.set(JointSelectionState::Y)
            }
            for _ in selection_over_event.read() { selection_state.set(JointSelectionState::N) }
            
            let mut dragged = false;
            
            ui.label("Tile position x");
            let res = ui.add(egui::DragValue::new(&mut translation.x).speed(0.06));
            if !dragged {
                dragged = res.dragged();
            }
            ui.label("Tile position y");
            let res = ui.add(egui::DragValue::new(&mut translation.y).speed(0.06));
            if !dragged {
                dragged = res.dragged();
            }
            ui.label("Tile position on atlas x");
            let mut lposx = selections.lpos.x;
            let res = ui.add(egui::DragValue::new(&mut lposx).speed(0.06));
            if !dragged {
                dragged = res.dragged();
            }
            if res.changed() {
                changed = true;
                // update position
                lposx = lposx.clamp(0., lmax - selections.ulrect.size().x);
                let d = lposx - selections.lpos.x;
                selections.lpos.x += d;
                selections.gpos.x += d * scale;
                // update rects
                selections.ulrect.min.x += d;
                selections.ulrect.max.x += d;
                selections.sgrect.min.x += d; // todo: check scale?
                selections.sgrect.max.x += d;
            }

            ui.label("Tile position on atlas y");
            let mut lposy = selections.lpos.y;
            let res = ui.add(egui::DragValue::new(&mut lposy).speed(0.06));
            if !dragged {
                dragged = res.dragged();
            }
            if res.changed() {
                changed = true;
                // update position
                lposy = lposy.clamp(0., lmay - selections.ulrect.size().y);
                let d = lposy - selections.lpos.y;
                selections.lpos.y += d;
                selections.gpos.y += d * scale;
                // update rects
                selections.ulrect.min.y += d;
                selections.ulrect.max.y += d;
                selections.sgrect.min.y += d;
                selections.sgrect.max.y += d;
            }
            
            ui.label("Width");
            let mut width = selections.ulrect.width();
            let res = ui.add(egui::DragValue::new(&mut width).speed(0.06));
            if !dragged {
                dragged = res.dragged();
            }
            if res.changed() {
                changed = true;
                // update size
                width = width.clamp(0., lmax - selections.lpos.x);
                let d = width - selections.ulrect.width();
                if d != 0. {
                    // update rects
                    selections.ulrect.max.x += d;
                    selections.sgrect.max.x += d * scale;
                }
            }

            ui.label("Height");
            let mut height = selections.ulrect.height();
            let res = ui.add(egui::DragValue::new(&mut height).speed(0.06));
            if !dragged {
                dragged = res.dragged();
            }
            if res.changed() {
                changed = true;
                // update size
                height = height.clamp(0., lmay - selections.ulrect.height());
                let d = height - selections.ulrect.height();
                // update rects
                selections.ulrect.max.y += d;
                selections.sgrect.max.y += d * scale;
            }
            if ui.ui_contains_pointer() || dragged {
                // todo: if pressed above ui, stop sending only in released
                // or remove the event and replace with other logic
                // main idea: if pressed on ui, ignore all other uis, while not released
                cursor_above_ui.send(CursorAboveUi);
            }
        });
        if changed {
            commands.entity(selections.entity).insert(
                SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(0.,0.,0.,),
                    sprite: Sprite {
                        rect: Some(selections.ulrect),
                        ..default()
                    },
                    ..default()
                },
            );
        }
    }
}