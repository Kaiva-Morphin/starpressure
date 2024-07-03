use bevy::math::{vec2, vec3};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::HashSet;
use bevy::{math::ivec2, math::uvec2, prelude::*};
use bevy::render::texture::{Image, ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
use bevy::sprite::TextureAtlas;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use std::cell::{RefCell, RefMut};
use std::sync::{Arc, Mutex};

use super::tiles::{MultitileType, TileData};

pub const PIXELS_PER_UNIT: f32 = 8.;

#[derive(Clone)]
struct StoredTile{
    entity: Entity,
    tile_data: Arc<TileData>,
    origin_offset: IVec2,
    overlay_tiles: Vec<StoredOverlay>, // todo: -> Vec<StoredOverlay>?
    variant: usize,
    neighborstate: usize,
    frame: usize,
}

impl StoredTile{
    pub fn iter_offsets(&self) -> impl Iterator<Item = IVec2> + '_ {
        self.tile_data.iter_offsets()
    }
    pub fn iter_neighbors_offsets(&self) -> impl Iterator<Item = (IVec2, usize)> + '_ {
        self.tile_data.iter_neighbors_offsets()
    }
    pub fn iter_offsets_masked(&self) -> impl Iterator<Item = IVec2> + '_ {
        self.tile_data.iter_offsets_masked()
    }
    pub fn is_multitile(&self) -> bool {
        self.tile_data.is_multitile()
    }
    pub fn is_origin(&self) -> bool {
        self.origin_offset == ivec2(0, 0)
    }

    pub fn get_frame(&self, elapsed_seconds: f32) -> usize {
        self.tile_data.get_frame(elapsed_seconds)
    }
    pub fn iter_around_tile(&self) -> impl Iterator<Item = &IVec2> {
        self.tile_data.iter_to_update()
    }
}

#[derive(Clone)]
struct StoredOverlay{
    entity: Entity
    //offset: UVec2
}

#[derive(Component)]
pub struct TileGrid{
    size: UVec2,
    singletile_size: Vec2,
    tiles: Mutex<RefCell<Vec<Vec<Option<StoredTile>>>>>,
    to_update: Mutex<RefCell<Vec<(UVec2, TileAction)>>>,
    entity: Entity,
}


#[derive(Clone)]
enum TileAction{
    Add{tile_data: Arc<TileData>},
    Remove
}

impl TileGrid {
    pub fn singletile_size(&self) -> Vec2 {
        return self.singletile_size
    }

    pub fn size(&self) -> UVec2 {
        return self.size
    }

    pub fn build_for_entity(e: Entity, commands: &mut Commands, size: UVec2, images: &mut ResMut<Assets<Image>>){
        let color = [0, 0, 0, 128];
        let texture_size = (16, 16);
        let mut texture_array: Vec<u8> = vec![0; texture_size.0 * texture_size.1 * 4];
        let p: usize = 3;
        
        let x = texture_size.0;
        let y = texture_size.1;

        for i in 0..p {
            let mut set_pixel = |pos: usize| {
                texture_array[pos*4..pos*4+4].copy_from_slice(&color);
            };
            // lu
            set_pixel(i);
            set_pixel(i*x);
            // ru
            set_pixel(x - i - 1);
            set_pixel((x - 1) + x * i);
            // rd
            set_pixel(x * y - (x * i) - 1);
            set_pixel(x * y - i - 1);
            // ld
            set_pixel(x * y - x + i);
            set_pixel(x * y - x - x* i);
        }

        let mut texture_image = Image::new(
            Extent3d {
                width: x as u32,
                height: y as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            texture_array,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(),
        );
        texture_image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor{
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            address_mode_w: ImageAddressMode::Repeat,
            ..default()
        });
        
        let texture = images.add(texture_image);
        let singletile_size = Vec2::splat(PIXELS_PER_UNIT);
        commands.entity(e).insert((TileGrid{
            size: size,
            singletile_size: singletile_size,
            tiles: Mutex::new(RefCell::new(vec![vec![None; size.y as usize]; size.x as usize])),
            to_update: Mutex::new(RefCell::new(vec![])),
            entity: e
        },
        VisibilityBundle::default(),
        TransformBundle::default(),
        Name::new("TileGrid"))).with_children(|b|{b.spawn(
            SpriteBundle{
                texture: texture,
                sprite: Sprite {
                    custom_size: Some(singletile_size * size.as_vec2()),
                    rect: Some(Rect{min: Vec2::ZERO, max: vec2(texture_size.0 as f32, texture_size.1 as f32) * size.as_vec2()}),
                    anchor: bevy::sprite::Anchor::Center,
                    ..Default::default()
                },
                transform: Transform::from_translation(vec3(0., 0., -0.1)),
                ..Default::default()
            });});
    }

    pub fn add(&self, pos: &IVec2, tile: Arc<TileData>){
        if !self.in_bounds_i(pos) {return;}
        let to_update = self.to_update.lock().unwrap_or_else(|err|{panic!("Cant lock tile grid's to_update field: {}", err);});
        to_update.borrow_mut().push((pos.as_uvec2(), TileAction::Add {tile_data: tile}));
    }

    pub fn set(&self, pos: &IVec2, tile: Option<Arc<TileData>>){
        if !self.in_bounds_i(pos) {return;}
        let to_update = self.to_update.lock().unwrap_or_else(|err|{panic!("Cant lock tile grid's to_update field: {}", err);});
        if let Some(tile) = tile {
            to_update.borrow_mut().push((pos.as_uvec2(), TileAction::Add {tile_data: tile}));
        } else {
            to_update.borrow_mut().push((pos.as_uvec2(), TileAction::Remove));
        }
    }

    fn get_sprite_pos(&self, pos: &UVec2) -> Vec3 {
        let grid_size = self.size.as_vec2() * self.singletile_size;
        (pos.as_vec2() * self.singletile_size - grid_size * 0.5).extend(0.)
    }

    fn get_sprite_pos_i(&self, pos: &IVec2) -> Vec3 {
        let grid_size = self.size.as_vec2() * self.singletile_size;
        (pos.as_vec2() * self.singletile_size - grid_size * 0.5).extend(0.)
    }

    fn get_tile<'a> (&'a self, tiles: &'a RefMut<Vec<Vec<Option<StoredTile>>>>, pos: &UVec2) -> Option<&Option<StoredTile>>{
        if !self.in_bounds(pos){return None} 
        if let Some(v) = tiles.get(pos.x as usize) {
            if let Some(tile_ref) = v.get(pos.y as usize) {
                return Some(tile_ref)
            }
        }
        None
    }

    fn get_tile_mut<'a> (&'a self, tiles: &'a mut RefMut<'_, Vec<Vec<Option<StoredTile>>>>, pos: &UVec2) -> Option<&mut Option<StoredTile>>{
        if !self.in_bounds(pos){return None} 
        if let Some(v) = tiles.get_mut(pos.x as usize) {
            if let Some(tile_ref) = v.get_mut(pos.y as usize) {
                return Some(tile_ref)
            }
        }
        None
    }

    fn set_tile(&self, pos: &UVec2, tile: &Arc<TileData>, commands: &mut Commands){
        let tiles = self.tiles.lock().unwrap_or_else(|err|{panic!("Cant lock tile grid's to_update field: {}", err);});
        let mut tiles = tiles.borrow_mut();

        if !self.can_be_placed(pos, tile, &mut tiles){return}

        let mut to_store = vec![];
        let mut overlay_tiles = vec![];

        
        let variant = if tile.chances.len() > 1 {
            let mut rng = thread_rng();
            let dist = WeightedIndex::new(tile.chances.clone()).unwrap();
            dist.sample(&mut rng)
        } else {0};

        let state = self.get_state(pos, tile, &tiles);
        if let Some(tile_ref) = self.get_tile_mut(&mut tiles, pos){ // if tile exists
            if let Some(stored_tile) = tile_ref {
                // update
                if stored_tile.tile_data == *tile {return}
                commands.entity(stored_tile.entity).insert((
                    SpriteBundle{
                        texture: tile.image.clone(),
                        sprite: Sprite {custom_size: Some(self.singletile_size), anchor:bevy::sprite::Anchor::BottomLeft, ..default()}, ..default()
                    },
                    TextureAtlas{
                        layout: tile.atlas_layouts.clone(),
                        index: tile.get_atlas_idx_from_neighborstate((0, 0), state, variant, 0)
                    }
                )).insert(Transform::from_translation(self.get_sprite_pos(pos)));
                *tile_ref = Some(StoredTile{entity: stored_tile.entity, tile_data: tile.clone(), origin_offset: IVec2::ZERO, overlay_tiles: vec![], neighborstate: state, variant: 0, frame: 0});
                warn!("Updated!");
                todo!();
            } else {
                // spawn
                
                for (pos_offset, index, z) in tile.iter_tile_atlas_idxs(state, variant, 0){
                    let ingrid_pos = pos.as_ivec2() + pos_offset;
                    // as single sprite!
                    let e = commands.spawn((
                        SpriteBundle{
                            texture: tile.image.clone(),
                            sprite: Sprite {custom_size: Some(self.singletile_size), anchor:bevy::sprite::Anchor::BottomLeft, ..default()}, ..default()
                        },
                        TextureAtlas{
                            layout: tile.atlas_layouts.clone(),
                            index
                        }
                    )).insert(Transform::from_translation(self.get_sprite_pos_i(&ingrid_pos) + vec3(0., 0., z))).set_parent(self.entity).id();
                    if z == 0. {
                        let origin = pos_offset == IVec2::ZERO;
                        to_store.push(
                            (ingrid_pos.as_uvec2(),
                            StoredTile{entity: e, tile_data: tile.clone(), origin_offset: pos_offset, overlay_tiles: vec![], neighborstate: state, variant: variant, frame: 0},
                            origin
                        ));
                    } else {
                        overlay_tiles.push(StoredOverlay{entity: e});
                    }
                }
            }
        } else {
            warn!("cant get tile")
        }
        for (pos, to_stored, is_origin) in to_store.iter_mut(){
            if let Some(tile_ref) = self.get_tile_mut(&mut tiles, pos){
                if *is_origin {
                    to_stored.overlay_tiles = overlay_tiles.clone();
                }
                *tile_ref = Some(to_stored.clone());
            }
        }
        let pos = pos.as_ivec2() + tile.origin_offset.as_ivec2();
        self.update_neighbor_tiles(&pos.as_uvec2(), tile.clone(), &mut tiles, commands);
    }

    fn can_be_placed<'a>(&'a self, origin: &UVec2, tile: &Arc<TileData>, tiles: &'a RefMut<Vec<Vec<Option<StoredTile>>>>) -> bool {
        let pos = origin.as_ivec2() + tile.origin_offset.as_ivec2() * ivec2(-1, 1);
        match &tile.multitile {
            MultitileType::Single => {
                if !self.in_bounds_i(&pos){return false};
                if let Some(tile) = self.get_tile(tiles, &pos.as_uvec2()){
                    if let Some(_stored_tile) = tile {
                        return false
                    }
                }
            }
            MultitileType::Multitile { z_mask } => {
                for (y, mask_y) in z_mask.iter().enumerate(){
                    for (x, z_layer) in mask_y.iter().enumerate(){
                        let rect = tile.get_main_layer_rect();
                        if !self.in_bounds_i(&(pos + rect.0.as_ivec2() * ivec2(1, -1))) || !self.in_bounds_i(&(pos + rect.1.as_ivec2() * ivec2(1, -1))){return false}
                        let pos = pos.as_uvec2();
                        if *z_layer != 0. {continue}
                        // todo: overflow sub handle
                        if let Some(tile) = self.get_tile(tiles, &(pos + uvec2(x as u32, 0) - uvec2(0, y as u32))){
                            if let Some(_stored_tile) = tile {
                                return false
                            }
                        }
                    }
                }
            }
        }
        true
    }
    
    /// pos is lu of tile!
    fn update_neighbor_tiles<'a>(&self, pos: &UVec2, tile: Arc<TileData>, mut tiles: &'a mut RefMut<Vec<Vec<Option<StoredTile>>>>, commands: &mut Commands){
        info!("\tUpdating neighbor tiles for {} with origin in {}", pos, pos.as_ivec2() - tile.origin_offset.as_ivec2());
        // prevent multiple update multitiles
        let mut updated_origins = HashSet::new();
        for offset in tile.iter_to_update() {
            let neighbor_pos = pos.as_ivec2() + *offset;
            info!("checkin {}({})", neighbor_pos, *offset);
            if !self.in_bounds_i(&neighbor_pos){continue}
            if let Some(stored_tile) = self.get_tile(tiles, &neighbor_pos.as_uvec2()){
                // if tile is here
                if let Some(stored_tile) = stored_tile {
                    // check is it need to be updated
                    //todo: autotile friends
                    if stored_tile.tile_data != tile {continue;}
                    if !stored_tile.tile_data.has_reaction() {continue;}
                    //updated
                    let neighbor_origin = neighbor_pos - stored_tile.origin_offset;
                    info!("origin {}", neighbor_origin);

                    if !self.in_bounds_i(&neighbor_origin){continue}
                    let neighbor_origin = neighbor_origin.as_uvec2();
                    if updated_origins.contains(&neighbor_origin){continue}
                    if let Some(neighbor_origin_tile) = self.get_tile(tiles, &neighbor_origin){
                        if let Some(neigbor) = neighbor_origin_tile{
                            if neigbor.tile_data != tile {continue;}
                            if !neigbor.is_origin() {continue;}
                            info!("update request send to {}", neighbor_origin);
                            self.update_neighbor(&neighbor_origin, &stored_tile.clone(), &mut tiles, commands);
                            updated_origins.insert(neighbor_origin);
                        }
                    }
                }
            }
        }
    }
    // origin_pos
    fn update_neighbor<'a>(&self, pos: &UVec2, tile: &StoredTile, tiles: &'a mut RefMut<Vec<Vec<Option<StoredTile>>>>, commands: &mut Commands){
        //tile.iter_offsets()
        let state = self.get_state(pos, &tile.tile_data, &tiles);
        if let Some(tile) = self.get_tile_mut(tiles, pos){
            if let Some(tile) = tile {
                tile.neighborstate = state;
            }
        }
        let frame = tile.frame;
        let variant = tile.variant;
        // update overlays
        for (i, idx) in tile.tile_data.iter_tile_atlas_idxs_for_overlay(state, variant, frame).enumerate(){
            if let Some(overlay_tile) = tile.overlay_tiles.get(i){
                commands.entity(overlay_tile.entity).insert(
                    TextureAtlas{
                        layout: tile.tile_data.atlas_layouts.clone(),
                        index: idx
                    }
                );
            } else {
                warn!("Overlay and StoredOverlay tiles has different size");
            }
        }
        // update main tiles
        for offset in tile.iter_offsets_masked(){
            let pos = pos.as_ivec2() + offset;
            if !self.in_bounds_i(&pos){continue}
            let pos = pos.as_uvec2();
            if let Some(tile) = self.get_tile_mut(tiles, &pos){
                if let Some(tile) = tile {
                    tile.neighborstate = state;
                    let relative = if IVec2::ZERO != tile.origin_offset {tile.tile_data.origin_offset_to_relative(tile.origin_offset)} else {tile.tile_data.origin_offset};
                    commands.entity(tile.entity).insert(
                        TextureAtlas{
                            layout: tile.tile_data.atlas_layouts.clone(),
                            index: tile.tile_data.get_atlas_idx_from_neighborstate((relative.x as usize, relative.y as usize), state, variant, frame)
                        }
                    );
                }
            }
        }
        


        let relative = if IVec2::ZERO != tile.origin_offset {tile.tile_data.origin_offset_to_relative(tile.origin_offset)} else {tile.tile_data.origin_offset};
        //let relative = if let Some(offset) = stored_tile.origin_offset {stored_tile.tile_data.origin_offset_to_relative(offset)} else {stored_tile.tile_data.origin_offset};
        commands.entity(tile.entity).insert(
            TextureAtlas{
                layout: tile.tile_data.atlas_layouts.clone(),
                index: tile.tile_data.get_atlas_idx_from_neighborstate((relative.x as usize, relative.y as usize), state, variant, frame)
            }
        );
        println!("{:?} UPDATED", pos);
    }

    fn get_state<'a>(&self, pos: &UVec2, tile: &Arc<TileData>, tiles: &'a RefMut<Vec<Vec<Option<StoredTile>>>>) -> usize{
        let mut state = 0;
        for (offset, influence) in tile.iter_neighbors_offsets() {
            //info!("{:?}", offset);
            let neighbor_pos = pos.as_ivec2() + offset;
            if !self.in_bounds_i(&neighbor_pos){continue}
            if let Some(stored_tile) = self.get_tile(tiles, &neighbor_pos.as_uvec2()){
                if let Some(stored_tile) = stored_tile {
                    if &stored_tile.tile_data != tile || !stored_tile.is_origin(){continue;}
                    state += influence;
                }
            }
        }
        state
    }

    fn despawn_tile(&self, pos: &UVec2, commands: &mut Commands){
        let tiles = self.tiles.lock().unwrap_or_else(|err|{panic!("Cant lock tile grid's to_update field: {}", err);});
        let mut tiles = tiles.borrow_mut();
        let mut to_erase = vec![];
        let mut erased_tile = None;
        let mut erased_tile_lu = IVec2::ZERO;
        if let Some(tile) = self.get_tile_mut(&mut tiles, pos) {
            if let Some(stored_tile) = tile {
            if stored_tile.is_multitile(){
                    erased_tile = Some(stored_tile.tile_data.clone());
                    erased_tile_lu = stored_tile.origin_offset - stored_tile.tile_data.origin_offset.as_ivec2();
                    println!("{}", erased_tile_lu);
                    for offset in stored_tile.iter_offsets_masked(){
                        to_erase.push(pos.as_ivec2() + offset - stored_tile.origin_offset);
                    }
                } else {
                    commands.entity(stored_tile.entity).despawn();
                    let erased_tile = tile.clone();
                    *tile = None;
                    if let Some(erased_tile) = erased_tile {
                        self.update_neighbor_tiles(pos, erased_tile.tile_data, &mut tiles, commands);
                    }
                    return;
                }
            }
        }
        for pos in to_erase.iter() {
            if !self.in_bounds_i(pos){continue;}
            let pos = pos.as_uvec2();
            if let Some(tile) = self.get_tile_mut(&mut tiles, &pos){
                if let Some(stored) = tile {
                    
                    for stored_overlay in stored.overlay_tiles.iter(){
                        commands.entity(stored_overlay.entity).despawn();
                    }
                    commands.entity(stored.entity).despawn();
                    *tile = None
                }
            }
        }
        if let Some(erased_tile) = erased_tile {
            self.update_neighbor_tiles(&(pos.as_ivec2() - erased_tile_lu).as_uvec2(), erased_tile, &mut tiles, commands);
        }
    }

    pub fn update(&mut self, commands: &mut Commands, time: &Res<Time>){
        let to_update = self.to_update.lock().unwrap_or_else(|err|{panic!("Cant lock tile grid's to_update field: {}", err);});
        let mut to_update = to_update.borrow_mut();
        for (pos, action) in to_update.iter() {
            if !self.in_bounds(pos) {continue;}
            match action {
                TileAction::Add { tile_data } => {
                    self.set_tile(pos, tile_data, commands);
                },
                TileAction::Remove => {
                    self.despawn_tile(pos, commands);
                }
            }
        }
        to_update.clear();
        self.update_animations(commands, time);
    }

    fn update_animations<'a>(&'a self, commands: &mut Commands, time: &Res<Time>){
        let tiles = self.tiles.lock().unwrap_or_else(|err|{panic!("Cant lock tile grid's to_update field: {}", err);});
        let mut tiles = tiles.borrow_mut();
        for stored_tile in self.iter_tiles_mut(&mut tiles){
            let frame = stored_tile.get_frame(time.elapsed_seconds());
            let variant = stored_tile.variant;
            let neighborstate = stored_tile.neighborstate;
            // update self and binded overlay tiles
            if stored_tile.is_origin(){
                for (i, idx) in stored_tile.tile_data.iter_tile_atlas_idxs_for_overlay(neighborstate, variant, frame).enumerate(){
                    if let Some(overlay_tile) = stored_tile.overlay_tiles.get(i){
                        commands.entity(overlay_tile.entity).insert(
                            TextureAtlas{
                                layout: stored_tile.tile_data.atlas_layouts.clone(),
                                index: idx
                            }
                        );
                    } else {
                        warn!("Overlay and StoredOverlay tiles has different size");
                    }
                }
            }
            stored_tile.frame = frame;
            // relative to origin -> relative to  multitile
            let relative = if IVec2::ZERO != stored_tile.origin_offset {stored_tile.tile_data.origin_offset_to_relative(stored_tile.origin_offset)} else {stored_tile.tile_data.origin_offset};
            //let relative = if let Some(offset) = stored_tile.origin_offset {stored_tile.tile_data.origin_offset_to_relative(offset)} else {stored_tile.tile_data.origin_offset};
            commands.entity(stored_tile.entity).insert(
                TextureAtlas{
                    layout: stored_tile.tile_data.atlas_layouts.clone(),
                    index: stored_tile.tile_data.get_atlas_idx_from_neighborstate((relative.x as usize, relative.y as usize), neighborstate, variant, frame)
                }
            );
        }
    }

    
    fn iter_tiles_mut<'a, 'b> (&'a self, tiles: &'b mut RefMut<Vec<Vec<Option<StoredTile>>>>) -> impl Iterator<Item = &'b mut StoredTile>{
        tiles.iter_mut().flat_map(|v|{
            v.iter_mut().flat_map(|tile| {
                tile.as_mut()
            })
        }) 
    }

    fn iter_tiles<'a>(&'a self, tiles: &'a std::cell::Ref<Vec<Vec<Option<StoredTile>>>>) ->  impl Iterator<Item = &StoredTile> {
        tiles.iter().flat_map(|v|{
            v.iter().flat_map(|tile| -> Option<&StoredTile>{
                tile.as_ref()
            })
        })
    }



    pub fn in_bounds_i(&self, pos: &IVec2) -> bool {
        !(pos.x < 0 || pos.y < 0 || pos.x as u32 >= self.size.x || pos.y as u32 >= self.size.y)
    }

    pub fn in_bounds(&self, pos: &UVec2) -> bool {
        !(pos.x as u32 >= self.size.x || pos.y as u32 >= self.size.y)
    }

    pub fn remove(&mut self, pos: &IVec2){
        if !self.in_bounds_i(pos) {return;}
        let to_update = self.to_update.lock().unwrap_or_else(|err|{panic!("Cant lock tile grid's to_update field: {}", err);});
        let mut to_update = to_update.borrow_mut();
        to_update.push(
            (pos.as_uvec2(), TileAction::Remove)
        );
    }

    pub fn draw_grid(&self, transform: Transform, gizmos: &mut Gizmos){
        /*
        gizmos.line(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, 1.0), Color::RED);
        */
        for x in 0..=self.size.x {
            for y in 0..=self.size.y {
                let transformed = transform * (vec3(x as f32, y as f32, 0.) * self.singletile_size.extend(0.));
                gizmos.line(transformed + self.singletile_size.extend(0.) * Vec3::X * 0.15, transformed - self.singletile_size.extend(0.) * Vec3::X * 0.15, Color::WHITE);
                gizmos.line(transformed + self.singletile_size.extend(0.) * Vec3::Y * 0.15, transformed - self.singletile_size.extend(0.) * Vec3::Y * 0.15, Color::WHITE);
            }
        }
    }

}