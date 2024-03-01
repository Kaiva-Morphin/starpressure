use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::WindowSize;

use super::components::UiVars;

pub fn update_tab(
    mut egui_context: EguiContexts,
    mut vars: ResMut<UiVars>,
    window_size: Res<WindowSize>,
) {
    let ctx = egui_context.ctx_mut();
    egui::Window::new(&vars.title)
    .default_width(200.)
    .default_height(800.)
    .vscroll(true)
    .resizable(true)
    .constrain(true)
    .default_pos(egui::Pos2::new(0., window_size.height as f32 * 0.2))
    .id(egui::Id::new(666))
    .show(ctx, |ui|{
        ui.add(egui::TextEdit::singleline(&mut vars.title).hint_text("Title"));
        //ui.add(Slider::new(value, range))
    })
    ;
}