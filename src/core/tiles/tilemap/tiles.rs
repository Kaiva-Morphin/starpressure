use bevy::math::ivec2;
use bevy::{math::uvec2, prelude::*};
use serde::{Deserialize, Serialize};
use bevy::asset::Handle;
use bevy::render::texture::Image;
use std::sync::Arc;

use super::parser::parse_folder;

pub const PIXELS_PER_UNIT: f32 = 8.;

/* todo:
Split to chunks
Baking?
Tile Trait? 
autotile with some other ids.
Random and ordered textures

Size: Single, Multi
Texture: Static, Animated, Switchable
Placement: Single, Random, Pattern
Neighbor reaction: None, Auto4, Auto8, 
Tags:

Texture "blending"
*/


// todo: switch to game/module resources?
// external resource needed for tilegrid update
#[derive(Default)]
#[derive(Resource)]
pub struct TilesCollections{
    collections: Vec<TileCollection>
}

impl TilesCollections{
    pub fn add(&mut self, atlas: TileCollection) -> usize {
        self.collections.push(atlas);
        return self.collections.len() - 1
    }
    pub fn get(&self, index: usize) -> Option<&TileCollection> {
        self.collections.get(index)
    }

    pub fn get_tile(&self, collection_id: usize, tile_id: usize) -> Option<Arc<TileData>> {
        if let Some(collection) = self.get(collection_id) {
            return collection.get(tile_id).cloned()
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<TileCollection> {
        self.collections.iter()
    }
    pub fn into_iter(&self) -> std::vec::IntoIter<TileCollection> {
        self.collections.clone().into_iter()
    }
}


#[derive(Default, Clone)]
pub struct TileCollection {
    name: String,
    tiles: Vec<Arc<TileData>>
}

impl TileCollection {
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    pub fn new(name: String) -> Self {
        TileCollection {name, ..default() }
    }
    pub fn from_folder(path: &str, assets: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,) -> Self {
        parse_folder(path, assets, texture_atlases)
    }

    pub fn get(&self, index: usize) -> Option<&Arc<TileData>> {
        self.tiles.get(index)
    }
    pub fn name(&self) -> String {
        return self.name.clone()
    }
    pub fn iter(&self) -> std::slice::Iter<Arc<TileData>>  {
        return self.tiles.iter()
    }
    pub fn into_iter(&self) -> std::vec::IntoIter<Arc<TileData>>  {
        return self.tiles.clone().into_iter()
    }
    pub fn append(&mut self, rh: &mut Vec<Arc<TileData>>){
        self.tiles.append(rh);
    }
    pub fn extend(&mut self, rh: &mut TileCollection){
        self.tiles.append(&mut rh.tiles);
    }
    pub fn extended(mut self, rh: &mut TileCollection) -> Self{
        self.extend(rh);
        self
    }
}

#[derive(Default)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TileTextureType {
    #[default]
    Static,
    Animated{delays: Vec<f32>},
    //Switchable
}

#[derive(Default)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TilePlacementType {
    #[default]
    Single,
    Random{chances: Vec<f32>},
    //Pattern
}

#[derive(Default)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[derive(TypePath)]
pub enum TileNeighborReaction {
    #[default]
    None,
    Auto4,
    Auto4Corners,
    RevAuto4, // for bg like in terraria
}


/*#[derive(Default)]
pub enum ReactionFriend {
    #[default]
    This,
    Any,
    NonThis,
    Specific(HashSet<TileUID>)
}*/



#[derive(Default)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MultitileType {
    #[default]
    Single,
    /// [
    ///  [a, b],
    ///  [c, d]
    /// ]
    /// z_mask[y][x]
    /// X and Y are swapped!
    Multitile{
        z_mask: Vec<Vec<f32>>
    }
}


#[derive(Default, Clone, PartialEq, Debug)]
#[derive(TypePath, Asset)]
pub struct TileData { // basic info about tile
    pub name: String,
    pub delays: Vec<f32>,
    pub chances: Vec<f32>, 
    pub multitile: MultitileType,
    pub neighbor_reaction: TileNeighborReaction,
    pub image: Handle<Image>,
    pub origin_offset: UVec2,
    pub atlas_layouts: Handle<TextureAtlasLayout>, // l[x][y], where x is variant, y is frame
    pub(super) size: UVec2,
    pub(super) atlas_columns: usize,
    pub(super) atlas_rows: usize,
    pub(super) main_layer_rect:  (UVec2, UVec2),
    pub(super) to_update: Vec<IVec2>,
}

impl TileData {
    pub fn iter_neighbors_offsets(&self) -> impl Iterator<Item = (IVec2, usize)> + '_ {
        self.neighbor_reaction.iter_offsets().map(|(offset, influence)|{
            (offset * self.get_main_layer_size().as_ivec2(), influence)
        })
    }

    pub fn iter_to_update(&self) -> impl Iterator<Item = &IVec2> + '_ {
        self.to_update.iter()
    }

    pub fn get_main_layer_size(&self) -> UVec2{
        self.main_layer_rect.1 - self.main_layer_rect.0 + uvec2(1, 1)
    }

    pub fn get_main_layer_rect(&self) -> (UVec2, UVec2){
        self.main_layer_rect
    }

    pub fn is_multitile(&self) -> bool {
        match self.multitile {
            MultitileType::Single => {false}
            MultitileType::Multitile { z_mask: _ } => {true}
        }
    }

    pub fn has_reaction(&self) -> bool {
        match self.neighbor_reaction {
            TileNeighborReaction::None => false,
            TileNeighborReaction::Auto4 => true,
            TileNeighborReaction::Auto4Corners => true,
            TileNeighborReaction::RevAuto4 => true,
        }
    }
    /// *       ^
    /// | y  => | y
    /// v       *
    /// also reversing y
    /// origin_offset => tile_space
    /// pos NOT in tile space
    pub fn origin_offset_to_relative(&self, offset: IVec2) -> UVec2 {
        (self.origin_offset.as_ivec2() + (offset * ivec2(1, -1))).as_uvec2()
    }
    
    /// *       ^
    /// | y  => | y
    /// v       *
    /// also reversing y
    /// tile_space => origin_offset
    /// pos in tile space!
    pub fn relative_to_origin_offset(&self, pos: UVec2) -> IVec2 {
        (pos.as_ivec2() - self.origin_offset.as_ivec2()) * ivec2(1, -1)
    }

    //pub fn get_reaction() -> u8 {}
    //pub fn get_atlas_i(){}
    //todo: baking?
    pub fn get_atlas_idx_from_neighborstate(&self, offset: (usize, usize), neighborstate: usize, variant: usize, frame: usize) -> usize {
        let pos = self.neighbor_reaction.get_pos_from_neighborstate(neighborstate);
        let pos = (pos.0 * self.size.x as usize, pos.1 * self.size.y as usize);
        self.atlas_columns * self.size.x as usize * frame + 
        variant * self.atlas_rows * self.size.y as usize * self.atlas_columns * self.size.x as usize * self.delays.len().max(1) + 
        self.delays.len().max(1) * self.atlas_columns * self.size.x as usize * (pos.1 + offset.1) + 
        (pos.0 + offset.0)
    }

    pub fn get_atlas_idx_from_multitile_pos(&self, pos: (usize, usize), variant: usize, frame: usize) -> usize {
        self.atlas_columns * self.size.x as usize * frame + 
        variant * self.atlas_rows * self.size.y as usize * self.atlas_columns * self.size.x as usize * self.delays.len().max(1) + 
        self.delays.len().max(1) * self.atlas_columns * self.size.x as usize * pos.1 + 
        pos.0
    }


    //todo: baking?
    pub fn get_frame(&self, elapsed_seconds: f32) -> usize {
        if self.delays.len() < 2 {return 0}
        let summ = self.delays.iter().sum();
        let elapsed_wrapped = elapsed_seconds.rem_euclid(summ);
        let mut accum = 0.;
        for (i, delay) in self.delays.iter().enumerate() {
            accum += delay;
            if accum > elapsed_wrapped {
                return i
            }
        }
        0
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }
    pub fn iter_offsets(&self) -> impl Iterator<Item = IVec2> + '_ {
        (0..self.size.x).into_iter().flat_map(move |x|{
            (0..self.size.y).into_iter().map(move |y|{
                ivec2(x as i32, -(y as i32)) - self.origin_offset.as_ivec2() * ivec2(1, -1)
            })
        })
    }
    pub fn iter_offsets_masked(&self) -> impl Iterator<Item = IVec2> + '_ {
        (0..self.size.x).into_iter().flat_map(move |x|{
            (0..self.size.y).into_iter().flat_map(move |y|{
                if self.multitile.get_z(uvec2(x, y)) == 0. {
                    return Some(ivec2(x as i32, -(y as i32)) - self.origin_offset.as_ivec2() * ivec2(1, -1))
                }
                None
            })
        })
    }
    //todo: baking?
    pub fn iter_tile_atlas_idxs(&self, neighborstate: usize, variant: usize, frame: usize) -> impl Iterator<Item = (IVec2, usize, f32)> + '_ {
        (0..self.size.x).into_iter().flat_map(move |x|{
            (0..self.size.y).into_iter().map(move |y|{
                // todo: check that (ivec2(x as i32, -(y as i32)) - self.origin_offset.as_ivec2() * ivec2(1, -1)   EQ   self.relative_to_origin_offset()
                (ivec2(x as i32, -(y as i32)) - self.origin_offset.as_ivec2() * ivec2(1, -1), self.get_atlas_idx_from_neighborstate((x as usize, y as usize), neighborstate, variant, frame), self.multitile.get_z(uvec2(x, y)))
            })
        })
    }
    pub fn iter_tile_atlas_idxs_for_overlay(&self, neighborstate: usize, variant: usize, frame: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.size.x).into_iter().flat_map(move |x|{
            (0..self.size.y).into_iter().flat_map(move |y|{
                if self.multitile.get_z(uvec2(x, y)) != 0. {
                    return Some(self.get_atlas_idx_from_neighborstate((x as usize, y as usize), neighborstate, variant, frame))
                }
                None
            })
        })
    }
}

impl MultitileType{
    pub fn get_z(&self, pos: UVec2) -> f32{
        match self {
            MultitileType::Single => {0.},
            MultitileType::Multitile { z_mask } => {
                if let Some(v) = z_mask.get(pos.y as usize){
                    if let Some(z) = v.get(pos.x as usize){
                        return *z
                    }
                }
                0.
            },
        }
    }
}

impl TileNeighborReaction {
    pub fn iter_offsets(&self) -> impl Iterator<Item = (IVec2, usize)> {
        match self {
            TileNeighborReaction::None => {
                vec![(ivec2(0, 0), 0)].into_iter()
            },
            TileNeighborReaction::Auto4 => {
                vec![
                    (ivec2(0, 1), 1),
                    (ivec2(1, 0), 2),
                    (ivec2(0, -1), 4),
                    (ivec2(-1, 0), 8)
                ].into_iter()
            },
            TileNeighborReaction::Auto4Corners => {
                todo!()
            },
            TileNeighborReaction::RevAuto4 => {
                todo!()
            },
        }
    }

    pub fn get_idx_from_neighborstate(&self, id: usize) -> usize {
        match self {
            TileNeighborReaction::None => {0},
            TileNeighborReaction::Auto4 => {
                *[12, 8, 13, 9, 0, 4, 1, 5, 15, 11, 14, 10, 3, 7, 2, 6].get(id).unwrap_or(&12)
            },
            TileNeighborReaction::Auto4Corners => {
                id.clamp(0, 11)
            },
            TileNeighborReaction::RevAuto4 => {
                /*
                rects!
                */
                0
            },
        }
    }
    pub fn get_pos_from_neighborstate(&self, id: usize) -> (usize, usize) {
        match self {
            TileNeighborReaction::None => {(0, 0)},
            TileNeighborReaction::Auto4 => {
                *[(0, 3), (0, 2), (1, 3), (1, 2), (0, 0), (0, 1), (1, 0), (1, 1), (3, 3), (3, 2), (2, 3), (2, 2), (3, 0), (3, 1), (2, 0), (2, 1)].get(id).unwrap_or(&(0, 3))
            },
            TileNeighborReaction::Auto4Corners => {
                (id % 6, id / 6)
            },
            TileNeighborReaction::RevAuto4 => {
                (1, 1)
            },
        }
    }
}
