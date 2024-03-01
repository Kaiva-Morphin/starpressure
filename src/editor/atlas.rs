use bevy::prelude::*;
use bevy_egui::{egui::{self, load::SizedTexture}, EguiContexts};
use bevy_file_dialog::DialogFileLoaded;

use crate::{components::WindowSize, AtlasFileContents};

pub fn update_atlas(
    mut egui_context: EguiContexts,
    //mut vars: ResMut<UiVars>,
    window_size: Res<WindowSize>,
    mut to_update: Local<bool>,
    loaded_atlas: Res<super::components::LoadedAtlas>,
) {
    if loaded_atlas.is_changed() {
        // todo: add logic for reloading atlas
    }
    let image = egui_context.image_id(&loaded_atlas.handle);
    let ctx = egui_context.ctx_mut();
    egui::Window::new("Texture Atlas")
    .default_width(800.)
    .default_height(600.)
    .vscroll(true)
    .resizable(true)
    .constrain(true)
    .default_pos(egui::Pos2::new(0., window_size.height as f32 * 0.2))
    .show(ctx, |ui|{
        if let Some(image_id) = image {
            ui.image(SizedTexture {
                id: image_id,
                size: egui::vec2(500.,500.,),
            });
        }
    });
        
    
}