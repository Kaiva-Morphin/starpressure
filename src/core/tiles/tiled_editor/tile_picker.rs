use std::sync::Arc;
use bevy::{asset::Assets, prelude::{Local, Res, ResMut, Resource}, sprite::TextureAtlasLayout};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui::{self, *}};
use load::SizedTexture;
use panel::TopBottomSide;

use crate::core::tiles::tilemap::tiles::{TileData, TilesCollections};

macro_rules! unwrap_or_continue {
    ($res:expr) => {
        match $res {
            Some(val) => val,
            None => {
                continue;
            }
        }
    };
}
#[derive(Default, Resource)]
pub struct PickedTile(pub Option<Arc<TileData>>);

pub fn menu(
    mut egui_context: EguiContexts,
    tile_collection: Option<Res<TilesCollections>>,
    mut picker_shown: Local<bool>,
    texture_assets: Res<Assets<bevy::prelude::Image>>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut picked: ResMut<PickedTile>,
    mut textures: Local<Vec<Vec<TextureId>>>,
){
    if textures.len() == 0 {
        *textures = if let Some(tile_collection) = &tile_collection {
            tile_collection.into_iter().map(|tile_atlas| -> Vec<TextureId> {
                tile_atlas.into_iter().map(|tile| -> TextureId{
                    // tiles
                    egui_context.add_image(tile.image.clone())
                }).collect()
            }).collect()
        } else {
            vec![]
        };
    }
    

    let ctx: &mut egui::Context = egui_context.ctx_mut();
    let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("Font".to_owned(),
        egui::FontData::from_static(crate::core::default::DEFAULT_FONT_BYTES )
    );
    fonts.families.insert(egui::FontFamily::Name("Font".into()), vec!["Font".to_owned()]);
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "Font".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .insert(0, "Font".to_owned());

    ctx.set_fonts(fonts);

    //let mut style = (*ctx.style()).clone();
    /*style.text_styles.insert(
        egui::TextStyle::Button, FontId::new(34.0, egui::FontFamily::Monospace)
    );
    style.text_styles.insert(
        egui::TextStyle::Body, FontId::new(34.0, egui::FontFamily::Monospace)
    );*/
    /*
    .text_styles = 
    */

    //ctx.set_style(style.clone());


    egui::TopBottomPanel::new(TopBottomSide::Top, "ToolsPanel")
        .show(ctx, |ui|{
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui|{
                let p = ui.button("Tile Picker");
                let _ = ui.button("New Grid");
                if p.clicked(){ *picker_shown = !*picker_shown; }
            });
    });
   

    egui::Window::new("PICKER")
        .vscroll(true)
        .hscroll(false)
        .open(&mut picker_shown)
        .show(ctx, |ui|{
            if let Some(tile_collection) = tile_collection {
                for (a, atlas) in textures.iter().enumerate(){
                    if let Some(tile_atlas) = tile_collection.get(a) {
                        ui.collapsing(tile_atlas.name(), |ui|{
                            ui.with_layout(egui::Layout::left_to_right(Align::Min).with_main_wrap(true), |ui|{
                                for (i, texture_id) in atlas.iter().enumerate(){
                                    if let Some(tile_data) = tile_atlas.get(i) {
                                        let texture = unwrap_or_continue!(texture_assets.get(tile_data.image.clone()));
                                        let layout = unwrap_or_continue!(atlas_layouts.get(tile_data.atlas_layouts.clone()));
                                        if layout.textures.is_empty(){continue;}

                                        let size = vec2(texture.width() as f32, texture.height() as f32);
                                        let r = ui.allocate_ui_with_layout(vec2(96., 96.), Layout::top_down(Align::Center), |ui|{
                                            
                                            let w = if picked.0 == Some(tile_data.clone()) {1.} else {0.};
                                            let frame = Frame::default().stroke(Stroke::new(w, Color32::WHITE));
                                            
                                            let texture_rect = layout.textures.get(tile_data.get_atlas_idx_from_neighborstate((0, 0), 0,  0, 0)).unwrap_or(&layout.textures[0]);
                                            
                                            let p1 = pos2(texture_rect.min.x, texture_rect.min.y);
                                            let p2 = pos2(texture_rect.max.x, texture_rect.max.y);

                                            let tile_size = p2 - p1;
                                            let multitile_size = tile_data.size().as_vec2(); /*match tile_data.multitile.clone() {
                                                MultitileType::Single => {vec2(1., 1.)}
                                                MultitileType::Multitile { z_mask } => {
                                                    vec2(z_mask[0].len() as f32, z_mask.len() as f32)
                                                }
                                            };*/
                                            let p2 = pos2(p1.x + tile_size.x * multitile_size.x, p1.y + tile_size.y * multitile_size.y);

                                            let rect = Rect::from_min_max(
                                                pos2(
                                                    p1.x / size.x,
                                                    p1.y / size.y,
                                                ) + vec2(0.002, 0.002),
                                                pos2(
                                                    p2.x / size.x,
                                                    p2.y / size.y,
                                                ) - vec2(0.002, 0.002)
                                            );
                                            //let rect = Rect::from_center_size(pos2(0., 0.), vec2(0.5, 0.5));
                                            frame.show(ui, |ui| {
                                                let r = ui.add(
                                                    ImageButton::new(
                                                    Image::new(
                                                        SizedTexture{id: *texture_id, size: p2-p1 }
                                                    ).uv(rect).fit_to_exact_size(vec2(96., 96.))
                                                    ).frame(false)
                                                );
                                                if r.clicked(){
                                                    picked.0 = Some(tile_data.clone());
                                                }
                                            });
                                            //shrink_to_fit();
                                            ui.label(tile_data.name.clone());
                                        });
                                        if r.response.clicked(){
                                            picked.0 = Some(tile_data.clone());
                                        }
                                    }
                                }
                            });
                        });
                    }
                }
            }
        });
    ctx.is_pointer_over_area();
}






