use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_egui::{egui::{self, load::SizedTexture}, EguiContexts};
use bevy_file_dialog::DialogFileLoaded;

use crate::{components::{CursorWorldPosition, WindowSize}, AtlasFileContents};

use super::components::{Atlas, AtlasData, Tile};

pub fn load_atlas(
    mut commands: Commands,
    mut load_event: EventReader<DialogFileLoaded<AtlasFileContents>>,
    asset_server: ResMut<AssetServer>,
    mut vars: ResMut<AtlasData>,
) {
    // todo: place the atlas somewhere in the middle
    for event in load_event.read() {
        if !vars.loaded {
            vars.loaded = true;
            let texture = asset_server.load(event.path.clone());
            let entity = commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    ..default()
                },
                Name::new("Atlas"),
                Atlas,
            )).id();
            vars.entity = entity;
            vars.image = texture;
        } else {
            println!("atlas is already loaded")
        }
    }
}


pub fn atlas_controller(
    mut commands: Commands,
    mut egui_context: EguiContexts,
    mut vars: ResMut<AtlasData>,
    image_handle_q: Query<&Handle<Image>, With<Atlas>>,
    images: Res<Assets<Image>>,
    mut transform_q: Query<&mut Transform, With<Atlas>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorWorldPosition>,
    mut selection_started: Local<bool>,
    mut selected_region: Local<Rect>,
    mut selected_position: Local<Vec2>,
    mut initial_position: Local<Option<Vec2>>,
    mut starter_settings: Local<bool>,
    mut draw: Local<bool>,
    mut gizmos: Gizmos,
) {
    // todo: add logic for reloading atlas
    if vars.loaded {
        let ctx = egui_context.ctx_mut();
        egui::Window::new("Atlas Controller")
        .vscroll(true)
        .resizable(true)
        .constrain(true)
        .show(ctx, |ui|{
            ui.label("Texture scale");
            let res = ui.add(egui::DragValue::new(&mut vars.scale).speed(0.03));
            ui.end_row();
            if let Ok(handle) = image_handle_q.get_single() {
                if let Some(img) = images.get(handle) {
                    let size = img.size_f32();
                    vars.size = size;
                    vars.rect.min = Vec2::new(vars.pos.x - size.x * 0.5 * vars.scale, vars.pos.y - size.y * 0.5 * vars.scale);
                    vars.rect.max = Vec2::new(vars.pos.x + size.x * 0.5 * vars.scale, vars.pos.y + size.y * 0.5 * vars.scale);
                }
                if res.changed() {
                    vars.changed = true;
                }
            }

            ui.label("Texture position x");
            let res = ui.add(egui::DragValue::new(&mut vars.pos.x).speed(0.06));
            ui.end_row();
            if res.changed() {
                vars.changed = true;
            }

            ui.label("Texture position y");
            let res = ui.add(egui::DragValue::new(&mut vars.pos.y).speed(0.06));
            ui.end_row();
            if res.changed() {
                vars.changed = true;
            }

        });
        if initial_position.is_none() {
            *initial_position = Some(vars.pos);
        }

        if vars.changed {
            // general update logic for atlas
            vars.changed = false;
            let mut transform = transform_q.single_mut();
            transform.translation = vars.pos.extend(0.);
            transform.scale = Vec3::new(vars.scale, vars.scale, 0.);
        }
        
        for (rect, rect_pos, _) in vars.selections.iter() {
            // draw already selected rects
            // todo: add rescale logic
            let area = Rectangle::from_corners(rect.min, rect.max);
            let d = vars.pos - initial_position.unwrap();
            gizmos.primitive_2d(
                area,
                *rect_pos + d,
                0.,
                Color::WHITE,
            )
        }

        if mouse_button_input.pressed(MouseButton::Left) {
            // selection logic
            // inside here selection region is in global transform
            // but is saved into atlasdata's local coords
            // todo: add ms someday
            // todo: interrupt if cursor inside egui window
            if *selection_started {
                selected_region.max = cursor_pos.pos;
                if *draw {
                    selected_region.max = selected_region.max.clamp(selected_region.min, vars.rect.max);
                    let area = Rectangle::from_corners(
                        selected_region.min,
                        selected_region.max
                    );
                    *selected_position = selected_region.min + area.half_size;
                    gizmos.primitive_2d(
                        area,
                        *selected_position,
                        0.,
                        Color::WHITE,
                    )
                }
            } else {
                let mut allowing_selection_rect = vars.rect;
                allowing_selection_rect.min -= Vec2::new(10., 10.);
                allowing_selection_rect.max += Vec2::new(10., 10.);
                if allowing_selection_rect.contains(cursor_pos.pos) {
                    *draw = true;
                    let point = cursor_pos.pos.clamp(vars.rect.min, vars.rect.max);
                    *selected_region = Rect::from_corners(point, point);
                    *selected_position = point;
                    *selection_started = true;
                }
            }
        } else {
            if *selection_started {
                *draw = false;
                *selection_started = false;
                let lpos = *selected_position - vars.pos;
                // center of selection in scaled atlas coords
                let lupos = lpos / vars.scale + vars.size / 2.;
                let mut luregion = selected_region.clone();
                luregion.min /= vars.scale;
                luregion.max /= vars.scale;
                luregion = Rect::from_center_size(
                    lupos,
                    Vec2::new(luregion.max.x - luregion.min.x, luregion.max.y - luregion.min.y)
                );
                let entity = commands.spawn((
                    SpriteBundle {
                        texture: vars.image.clone(),
                        transform: Transform::from_xyz(0.,0.,0.,),
                        sprite: Sprite {
                            rect: Some(luregion),
                            ..default()
                        },
                        ..default()
                    },
                    Tile::default(),
                )).id();
                vars.selections.push((*selected_region, lpos, entity));
            }
        }
        if mouse_button_input.pressed(MouseButton::Right) && vars.rect.contains(cursor_pos.pos) {
            // if moved or rescaled => :(
            for (id, (rect, _, _)) in vars.selections.iter().enumerate() {
                if rect.contains(cursor_pos.pos) {
                    vars.selected = Some(id);
                    break;
                }
            }
        }
        if !*starter_settings {
            *starter_settings = true;
            vars.pos = Vec2::new(200., 100.);
            vars.scale = 5.;
            vars.changed = true;
        }
    }
}