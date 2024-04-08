use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_egui::{egui::{self, load::SizedTexture}, EguiContexts};
use bevy_file_dialog::DialogFileLoaded;
use rand_chacha::rand_core::le;

use crate::{components::{CursorWorldPosition, WindowSize}, ragdoll::components::Joint, AtlasFileContents};

use super::{components::{Atlas, AtlasData, CursorAboveUi, JointSelectionOver, RagdollTile, Selection}, JointSelectionState};

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
    selection_state: Res<State<JointSelectionState>>,
    mut selection_over_event: EventWriter<JointSelectionOver>,
    mut gizmos: Gizmos,
    mut cursor_above_ui: EventReader<CursorAboveUi>,
    locals: (
        Local<bool>,
        Local<Rect>,
        Local<Vec2>,
        Local<Option<Vec2>>,
        Local<bool>,
        Local<bool>,),
) {
    if mouse_button_input.pressed(MouseButton::Middle) {
        println!("curs {:?}", cursor_pos.pos);
    }
    // todo: add logic for reloading atlas
    if vars.loaded {
        let mut interrupt = false;
        for _ in cursor_above_ui.read() { interrupt = true; }
        let mut selection_started = locals.0;
        let mut selected_region = locals.1;
        let mut selected_position = locals.2;
        let mut initial_position = locals.3;
        let mut starter_settings = locals.4;
        let mut draw = locals.5;

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
                    vars.rect.min = Vec2::new(vars.pos.x - size.x * 0.5 * vars.scale, vars.pos.y - size.y * 0.5 * vars.scale); // !!!!
                    vars.rect.max = Vec2::new(vars.pos.x + size.x * 0.5 * vars.scale, vars.pos.y + size.y * 0.5 * vars.scale); // !!!!
                }
                if res.changed() {
                    vars.changed = true;
                }
            }

            ui.label("Texture center position x");
            let res = ui.add(egui::DragValue::new(&mut vars.pos.x).speed(0.06));
            ui.end_row();
            if res.changed() {
                vars.changed = true;
            }

            ui.label("Texture center position y");
            let res = ui.add(egui::DragValue::new(&mut vars.pos.y).speed(0.06));
            ui.end_row();
            if res.changed() {
                vars.changed = true;
            }

        });

        if vars.changed {
            // general update logic for atlas
            vars.changed = false;
            let mut transform = transform_q.single_mut();
            transform.translation = vars.pos.extend(0.);
            transform.scale = Vec3::new(vars.scale, vars.scale, 0.);
        }
        
        for selection in vars.selections.values() {
            // draw already selected rects
            // todo: add rescale logic
            let rectangle = Rectangle::from_corners(selection.sgrect.min, selection.sgrect.max);
            let d = vars.pos - initial_position.unwrap();
            gizmos.primitive_2d(
                rectangle,
                selection.gpos + d + selection.sgrect.half_size(),
                0.,
                Color::WHITE,
            );
            
            let parent_gpos = selection.gpos;
            for (child_entity, joint) in selection.joints.iter() {
                let parent_pos = parent_gpos + joint.origin1;
                let child_pos = vars.selections[child_entity].gpos + joint.origin2;
                let direction = child_pos - parent_pos;
                let length = direction.length();
                let segment = Segment2d::new(Direction2d::new(direction / length).unwrap(), length);
                let origin = parent_pos + direction / 2. + d;
                gizmos.primitive_2d(
                    segment,
                    origin,
                    0.,
                    Color::BLUE
                );
            };
        }

        if !interrupt
        {if mouse_button_input.pressed(MouseButton::Left) {
            // selection logic
            // inside here selection region is in global transform
            // but is saved into atlasdata's local coords
            // todo: add ms someday
            if *selection_started {
                selected_region.max = cursor_pos.pos;
                if *draw {
                    selected_region.max = selected_region.max.clamp(selected_region.min, vars.rect.max);
                    let area = Rectangle::from_corners(
                        selected_region.min,
                        selected_region.max
                    );
                    *selected_position = selected_region.min;
                    gizmos.primitive_2d(
                        area,
                        *selected_position + area.half_size,
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
            if *selection_started && selected_region.width() > 0.5 && selected_region.height() > 0.5 {
                // saves selected region in unscaled local atlas coords
                *draw = false;
                *selection_started = false;
                let mut luregion = selected_region.clone();
                luregion.min -= vars.pos;
                luregion.max -= vars.pos;
                luregion.min /= vars.scale;
                luregion.max /= vars.scale;

                // CENTER of selection in UNSCALED LOCAL atlas coords
                let lupos = luregion.min + vars.size / 2.;                
                luregion = Rect::from_center_size(
                    lupos + luregion.half_size(),
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
                    RagdollTile::default(),
                )).id();
                println!("locals {:?} {:?}", luregion, lupos);
                vars.selections.insert(entity, Selection::new(
                    luregion,
                    *selected_region,
                    lupos,
                    *selected_position,
                    entity));
            }
        }
        if mouse_button_input.just_released(MouseButton::Right) && vars.rect.contains(cursor_pos.pos) {
            let selected_joint1 = vars.selected_joint1;
            let mut selection_finished = false;
            for (selection_entity, selection) in vars.selections.iter_mut() {
                if selection.sgrect.contains(cursor_pos.pos) {
                    match selection_state.get() {
                        JointSelectionState::Y => {
                            let rel_cursor_pos = cursor_pos.pos - selection.gpos;
                            if selected_joint1.is_none() {
                                vars.selected_joint1 = Some((selection.entity, rel_cursor_pos));
                            } else {
                                if selection.entity != selected_joint1.unwrap().0 {
                                    vars.selected_joint2 = Some((selection.entity, rel_cursor_pos));
                                    selection_finished = true;
                                }
                            }
                        }
                        JointSelectionState::N => {
                            vars.selected = Some(*selection_entity);
                        }
                    }
                    break;
                }
            }
            if selection_finished {
                let joint1 = vars.selected_joint1.unwrap();
                let joint2 = vars.selected_joint2.unwrap();
                let parent = vars.selections.get_mut(&joint1.0).unwrap();
                parent.joints.insert(
                    joint2.0, // path to child
                    Joint {
                        origin1: joint1.1, // origin of parent joint
                        origin2: joint2.1, // origin of child joint
                    }
                );
                let child = vars.selections.get_mut(&joint2.0).unwrap();
                child.parents.insert(joint1.0);
                vars.selected_joint1 = None;
                vars.selected_joint2 = None;
                selection_over_event.send(JointSelectionOver);
            }
        }}
        if !*starter_settings {
            *starter_settings = true;
            vars.pos = Vec2::new(200., 100.);
            vars.scale = 5.;
            vars.changed = true;
        }
        if initial_position.is_none() {
            *initial_position = Some(vars.pos);
        }
    }
}