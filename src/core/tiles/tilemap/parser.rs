use bevy::{math::{ivec2, uvec2, vec2}, prelude::*, utils::hashbrown::HashSet};
use serde::{Deserialize, Serialize};
use bevy::asset::Handle;
use bevy::render::texture::Image;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;
use std::sync::Arc;

use crate::core::tiles::tilemap;

use super::tiles::{MultitileType, TileCollection, TileData, TileNeighborReaction};

pub const PIXELS_PER_UNIT: f32 = 8.;





#[derive(Debug, Serialize, Deserialize, Clone)]
struct JsonTileData {
    /// name of tile
    name: String,
    /// pixels per tile
    tile_pixels: (u16, u16),
    /// if given, multitile_type is assigned to Multitile. If 0 given, that will prevent tile placment on that pos! At least one tile must be 0 and origin is must be assigned to 0.
    multitile_z_mask:  Option<Vec<Vec<i8>>>,
    //// if given, multitile_type is assigned to Multitile, if 1 (true), it will prevent tile placement on that pos !!! ORIGIN TILE POS WILL BE AUTO ASSIGNED TO 'true' !!!
    //#[serde(default, deserialize_with = "bools_from_ints")]
    //multitile_mask:  Option<Vec<Vec<bool>>>,
    /// left upper corner offset of "origin" tile when its multitile
    origin: Option<(u16, u16)>,
    /// if given, will use default layout patterns
    reaction: Option<TileNeighborReaction>,
    /// if given, vec.size is number of variants and texture_pacement is assigned to animated
    variant_chance: Option<Vec<f32>>,
    /// if given, vec.size is number of frames and texture_type is assigned to animated
    animation_time: Option<Vec<f32>>,
    /// used for multiple tiles in one png
    layout_offset: Option<(u16, u16)>
}

pub fn parse_tile(json: String, texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>, img: Handle<Image>) -> Option<Vec<TileData>> {
    let res = serde_json::from_str::<Vec<JsonTileData>>(json.as_str());
    if let Ok(result) = res {
            //println!("{:#?}", result)
            let mut tiles: Vec<TileData> = vec![];
            for json_data in result.iter(){
                let mut tile = TileData::default();
                let json_data = json_data.clone();
                let mut variants = 1;
                let mut frames = 1;
                tile.name = json_data.name;

                if let Some(delays) = json_data.animation_time {
                    frames = delays.len();
                    tile.delays = delays;
                }
                let size = if let Some(vec) = json_data.multitile_z_mask.clone() {
                    if let Some(col) = vec.get(0) {
                        (col.len(), vec.len())
                    } else {
                        (1, 1)
                    }
                } else {(1, 1)};
                let origin = json_data.origin.unwrap_or_default();
                tile.origin_offset = uvec2(origin.0 as u32, origin.1 as u32);
                //println!("{:?}", size);
                // todo: check that is origin is 0 and, if not assign to 0
                let mut rect = (uvec2(0, 0),uvec2(size.0 as u32 - 1, size.1 as u32 - 1));
                let around = vec![
                    ivec2(-1, -1),
                    ivec2(-1, 0),
                    ivec2(-1, 1),
                    ivec2(0, -1),
                    ivec2(0, 1),
                    ivec2(1, -1),
                    ivec2(1, 0),
                    ivec2(1, 1)
                ];
                let mut to_update_around: Vec<IVec2> = around.clone();
                if size.0 > 1 || size.1 > 1 {
                    let mut first_encountered_y = None;
                    let mut last_encountered_y = 0;
                    let mut min_x: Option<usize> = None;
                    let mut max_x = 0;

                    let mut update = HashSet::new();
                    let mut not_update = HashSet::new();

                    tile.multitile = MultitileType::Multitile {
                        z_mask: json_data.multitile_z_mask.unwrap_or(
                            vec![vec![0; size.0]; size.1]
                        ).into_iter().enumerate().map(
                            |(y, v)| -> Vec<f32> {
                                v.into_iter().enumerate().map(|(x, v)|{
                                    if v == 0 {
                                        if first_encountered_y.is_none(){first_encountered_y = Some(y);};
                                        last_encountered_y = y;
                                        if let Some(mx) = min_x {
                                            min_x = Some(mx.min(x));
                                        } else {
                                            min_x = Some(x)
                                        }
                                        max_x = max_x.max(x);

                                        if update.contains(&ivec2(x as i32, y as i32 * -1)) {
                                            update.remove(&ivec2(x as i32, y as i32 * -1));
                                        }
                                        not_update.insert(ivec2(x as i32, y as i32 * -1));

                                        for offset in around.iter() {
                                            let pos = ivec2(x as i32, y as i32 * -1) + *offset * ivec2(1, -1);
                                            if !not_update.contains(&pos) {
                                                update.insert(pos); 
                                            }
                                        }

                                    }
                                    let value = if origin == (x as u16, y as u16){0.}else{(v as f32) * 1./128.};
                                    value
                                }).collect()
                            }
                        ).collect()
                    };
                    rect = (uvec2(min_x.unwrap_or_default() as u32, first_encountered_y.unwrap_or_default() as u32), uvec2(max_x as u32, last_encountered_y as u32));
                    to_update_around = update.into_iter().collect();
                }
                info!("{:?}", to_update_around);
                warn!("Rect: from {} to {}", rect.0, rect.1);
                tile.to_update = to_update_around;
                tile.neighbor_reaction = json_data.reaction.unwrap_or_default();
                let (columns, rows) = match tile.neighbor_reaction {
                    tilemap::tiles::TileNeighborReaction::None => {
                        (1, 1)
                    },
                    tilemap::tiles::TileNeighborReaction::Auto4 => {
                        (4, 4)
                    },
                    tilemap::tiles::TileNeighborReaction::Auto4Corners => {todo!()},
                    tilemap::tiles::TileNeighborReaction::RevAuto4 => {
                        (3, 3)
                    },
                };
                if let Some(chances) = json_data.variant_chance {
                    variants = chances.len();
                    tile.chances = chances;
                }
                let tile_size = vec2(json_data.tile_pixels.0 as f32, json_data.tile_pixels.1 as f32);
                let offset: Option<Vec2> = if let Some(offset) = json_data.layout_offset {
                    Some(vec2(offset.0 as f32, offset.1 as f32) * tile_size)
                } else {
                    None
                };

                let texture_atlas = TextureAtlasLayout::from_grid(
                    tile_size, size.0 * columns * frames, size.1 * rows * variants, None, offset
                );
                tile.main_layer_rect = rect;
                tile.atlas_layouts = texture_atlases.add(texture_atlas);
                tile.atlas_columns = columns;
                tile.atlas_rows = rows;
                tile.size = uvec2(size.0 as u32, size.1 as u32);
                tile.image = img.clone();
                tiles.push(tile);
            }
            return Some(tiles)
        }
    return None
}

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

fn get_last_folder(path: &Path) -> Option<&str> {
    path.components().last().and_then(|component| {
        if let std::path::Component::Normal(os_str) = component {
            os_str.to_str()
        } else {
            None
        }
    })
}

pub fn parse_folder(
    path: &str,
    assets: &Res<AssetServer>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> TileCollection {
    let mut collection = TileCollection::new(get_last_folder(Path::new(path)).unwrap_or("untitled").to_string());
    let dir = fs::read_dir(path);

    if dir.is_err() {
        println!("Warn! cant read dir {}!", path);
        return collection;
    }
    
    let dir = dir.unwrap();
    for entry in dir {
        if entry.is_err(){continue;}
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() {continue;}
        let ext = unwrap_or_continue!(path.extension());
        if ext != "json" {continue;}
        let png_file = path.with_extension("png");
        let name = unwrap_or_continue!(path.file_name()).to_owned();
        if !png_file.exists(){
            println!("Warn! png for {:?} doesnt exists!", name);
            continue;
        }

        let file = fs::File::open(path);
        if file.is_err(){
            println!("Warn! cant read {:?} file!", name);
            continue;
        }
        let mut file = file.unwrap();
        let mut contents = String::new();
        
        if let Err(_) = file.read_to_string(&mut contents){
            println!("Warn! cant read to string from {:?} file!", name);
            continue;
        }
        
        let png_file = if png_file.is_absolute() {
            png_file
        } else {
            let components: Vec<&str> = unwrap_or_continue!(png_file.to_str()).split('/').collect();
            if components.len() > 2 {
                PathBuf::from(Path::new(&(components[2..].join("/"))))
            } else {
                continue;
            }
        };

        let img: Handle<Image> = assets.load(png_file);
        let parsed = parse_tile(contents, &mut texture_atlases, img);
        if parsed.is_none(){
            warn!("{:?} ERR!", name);
            continue;
        }
        info!("{:?} OK!", name);
        collection.append(&mut parsed.unwrap().into_iter().map(|v| -> Arc<TileData> {Arc::new(v)}).collect());
    }
    collection
}




/*
a is multitile x
b is multitile y
Xa by Xb -> Variants for animation
|
v
Variants for specific tiles

None Layout:
1a by 1b
Auto4 Layout:
4a by 4b
*/


