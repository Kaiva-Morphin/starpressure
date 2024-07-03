use bevy::{
    ecs::world, math::uvec2, prelude::*, window::PrimaryWindow
};

//use crate::core::{diagnostics_screen::{self, plugin::ScreenDiagnostics}, tiled_editor::tile_picker, tilemap::{tile_grid::TileGrid, tiles::{TileCollection, TilesCollections}}};

//use tiles::{PIXELS_PER_UNIT};

use crate::{core::tiles::tilemap::{tile_grid::TileGrid, tiles::{TileCollection, TilesCollections}}, debug::diagnostics_screen::{self, plugin::ScreenDiagnostics}};

use super::tile_picker::{self, PickedTile};
pub struct TilemapEditorPlugin;


impl Plugin for TilemapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(
                Update,
                (
                    update,
                    tile_picker::menu
                )
            );
        app.insert_resource(PickedTile::default());
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
){
    let mut collection = TilesCollections::default();    
    collection.add(TileCollection::from_folder("./assets/starpressure", &assets, &mut texture_atlases).with_name("Starpressure"));
    collection.add(TileCollection::from_folder("./assets/blocks/floor", &assets, &mut texture_atlases).with_name("Pocket Surivial").extended(&mut TileCollection::from_folder("./assets/blocks/walls", &assets, &mut texture_atlases)));
    commands.insert_resource(collection);
    let e = commands.spawn_empty().id();
    TileGrid::build_for_entity(e, &mut commands, uvec2(10, 10), &mut images);
    commands.entity(e).insert(Transform::from_scale(Vec3::splat(2.)));
}

fn update(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut grid: Query<(&mut TileGrid, &Transform)>,
    picked: Res<PickedTile>,
    time: Res<Time>
){
    let (mut grid, transform) = grid.single_mut();
    //grid.draw_grid(*transform, &mut gizmos);
    let l = buttons.pressed(MouseButton::Left);
    let r = buttons.pressed(MouseButton::Right);
    if  l || r {
        let (camera, camera_transform) = q_camera.get_single().expect("Err with camera query!");
        let window = q_window.get_single().expect("Err with window query!");
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {   
            let pos = ((world_position.extend(0.) / transform.scale).truncate() / grid.singletile_size() + grid.size().as_vec2() * 0.5).floor().as_ivec2();
            if l {
                    grid.set(&pos, picked.0.clone());
            }
            if r {
                grid.set(&pos, None);
            }
        } else {
            //warn!("Err on pos convert!");
        }
    }
    let t = std::time::Instant::now();
    grid.update(&mut commands, &time);
}
