use bevy::{asset::{AssetServer, Assets, Handle}, core::Name, ecs::{bundle::Bundle, component::Component, entity::Entity, system::{Commands, Res, ResMut, Resource}, world::World}, gizmos::gizmos::Gizmos, hierarchy::{BuildChildren, ChildBuilder}, log::warn, math::{IVec2, UVec2, Vec2, Vec3, Vec4}, prelude::default, render::{color::Color, render_asset::RenderAssetUsages, render_resource::{Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, texture::{CompressedImageFormats, Image, ImageSampler, ImageType}}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, transform::components::Transform, utils::hashbrown::HashMap};

/*
pub struct TileCollisionShapes{
    
}
*/




// todo: this enum for test only, add multitile generator and switch to struct! (this may be used as tile_type)
// generator must put multitile pieces in end of vec, out of base len.
#[derive(Clone)]
pub enum Tile{ 
    Singletile,
    MultitileOrigin{parts_offsets_ids: Vec<(IVec2, usize)>},
    Multitile{origin_offset: IVec2, origin_id: usize}
}

/*
pub struct Tile {
    durability: ?,
    airflow: f32, // 0 -> 1, ability of tile to pass air
    waterflow: f32,
    collision_shape: ???,
    origin_offset: IVec2,
    parts_offsets: Vec<IVec2>,

}
*/



impl Default for Tile {
    fn default() -> Self {
        return Tile::Singletile
    }
}




// name <-> tile <-> Vec< atlas_id Vec< tile_id > >
#[derive(Clone)]
pub struct TileSet{
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
    pub tiles: Vec<Tile> // id -> tile
}

#[derive(Resource, Clone)] 
pub struct TileSetCollection{ // tileset names? // todo: make global singletone if possible
    pub tilesets: Vec<TileSet>,
    //name_bindings: HashMap<String, Tile>, // make something better!
}
/*impl Default for TileSetCollection{
    fn default() -> Self {
        TileSetCollection{
            tilesets: vec![],
            //name_bindings: HashMap::new(),
        }
    }
}*/
impl TileSetCollection{
    pub fn init(texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>, asset_server: &Res<AssetServer>) -> Self{
        let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(0.0, 0.0), 1, 1, None, None);
        let layout: Handle<TextureAtlasLayout> = texture_atlases.add(texture_atlas);
        let size = Extent3d {
            width: 1,
            height: 1,
            ..default()
        };
        let mut image: Image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8Unorm,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            ..default()
        };

        image.resize(size);

        
        let image_handle = asset_server.add(image);
        TileSetCollection{
            tilesets: vec![TileSet{
                texture: image_handle,
                layout: layout,
                tiles: vec![Tile::Singletile]
            }]
        }
    }

    pub fn register_new_tileset(
        &mut self,
        atlas_layout: Handle<TextureAtlasLayout>,
        texture: Handle<Image>,
    ) -> usize{
        self.tilesets.push(TileSet{
            layout: atlas_layout,
            texture: texture,
            tiles: vec![]
        });
        return self.tilesets.len() - 1; // id of tileset
    }

    pub fn set_tiles( // overrides existing tiles!
        &mut self,
        tileset_id: usize,
        tiles: Vec<Tile>
    ) -> bool {
        let tileset = self.tilesets.get_mut(tileset_id);
        if let Some(tileset) = tileset{
            tileset.tiles = tiles;
        } else {
            warn!("Tileset with id {} desnt exists!", tileset_id);
            return false;
        }
        return true;
    }

    pub fn get_tile(
        &self,
        tileset_id: usize,
        tile_id: usize,
    ) -> Option<&Tile> {
        if let Some(tileset) = self.tilesets.get(tileset_id){
            return tileset.tiles.get(tile_id)
        }
        return None;
    }
    
}



struct TileID{
    tileset_id: usize,
    tile_id: usize
}

#[derive(Component)]
pub struct TileMap{ // todo: tilemap ship bundle!
    size: UVec2, // left down is 0, 0 corner
    tiles: Vec<Vec<Entity>>, 

    //bg_tiles: Vec<Vec<usize>>,
}


impl TileMap{
    pub fn init_for(e: Entity, size: UVec2, commands: &mut Commands, collection: &TileSetCollection){ // todo: switch to bulder!
        let mut tiles = vec![];
        commands.entity(e).with_children(|children_builder|{
            let air_tileset = collection.tilesets.get(0).unwrap();
            
            for x in 0..size.x{
                let mut row = vec![];
                for y in 0..size.y{
                    let e = children_builder.spawn((
                        Name::from(String::from("TILE")),
                        SpriteSheetBundle {
                            transform: Transform {
                                scale: Vec3::splat(8.0),
                                translation: Vec3{x: x as f32 * 64., y: y as f32 * 64., z: 0.},
                                ..default()
                            },
                            texture: air_tileset.texture.clone(), // ????
                            atlas: TextureAtlas {
                                index: 0,
                                layout: air_tileset.layout.clone(), // ????
                            },
                            ..default()
                        }
                    )).id();
                    row.push(e);
                }
                tiles.push(row);
            }
        }).insert(TileMap{size, tiles});
    }

    pub fn draw_grid(
        &self,
        gizmos: &mut Gizmos,
        transform: &Transform
    ){

        for x in 0..=self.size.x{
            gizmos.line_2d(
                (transform.compute_matrix().transform_point3(Vec3{x: x as f32 * 64. - 32., y: 0. - 32., z: 0.})).truncate(),
                (transform.compute_matrix().transform_point3(Vec3{x: x as f32 * 64. - 32., y: self.size.y as f32 * 64. - 32., z: 0.})).truncate(),
                Color::BLACK
            );
            for y in 0..=self.size.y{
                gizmos.line_2d(
                    (transform.compute_matrix().transform_point3(Vec3{x: 0. - 32., y: y as f32 * 64. - 32., z: 0.})).truncate(),
                    (transform.compute_matrix().transform_point3(Vec3{x: self.size.x as f32 * 64. - 32., y: y as f32 * 64. - 32., z: 0.})).truncate(),
                    Color::BLACK
                );
            }
        }
    }

    pub fn set_tile(
        &mut self,
        commands: &mut Commands,
        position: UVec2,
        collection: &TileSetCollection,
        tileset_id: usize,
        tile_id: usize
    ) -> bool {
        let tileset = collection.tilesets.get(tileset_id).unwrap();
        let tile = collection.get_tile(tileset_id, tile_id).unwrap();
        let e = self.tiles.get(position.x as usize).unwrap().get(position.y as usize).unwrap();

        match tile{
            Tile::Singletile => {
                commands.entity(*e).insert(
                    SpriteSheetBundle {
                        transform: Transform {
                            scale: Vec3::splat(8.0),
                            translation: Vec3{x: position.x as f32 * 64., y: position.y as f32 * 64., z: 0.},
                            ..default()
                        },
                        texture: tileset.texture.clone(), // ????
                        atlas: TextureAtlas {
                            index: tile_id,
                            layout: tileset.layout.clone(), // ????
                        },
                        ..default()
                    }
                );
                return true;
            }
            Tile::Multitile { origin_offset, origin_id } => {
                // todo: ignore?
                return false;
            }
            Tile::MultitileOrigin { parts_offsets_ids } => {
                for (offset, id) in parts_offsets_ids.iter(){
                    let part_pos = IVec2{x: position.x as i32 + offset.x, y: position.y as i32 + offset.y};
                    if part_pos.x >= 0 && part_pos.y >= 0 && part_pos.x < self.size.x as i32 && part_pos.y < self.size.y as i32 {
                        let e = self.tiles.get(part_pos.x as usize).unwrap().get(part_pos.y as usize).unwrap();
                        commands.entity(*e).insert(
                            SpriteSheetBundle {
                                transform: Transform {
                                    scale: Vec3::splat(8.0),
                                    translation: Vec3{x: part_pos.x as f32 * 64., y: part_pos.y as f32 * 64., z: 0.},
                                    ..default()
                                },
                                texture: tileset.texture.clone(), // ????
                                atlas: TextureAtlas {
                                    index: *id,
                                    layout: tileset.layout.clone(), // ????
                                },
                                ..default()
                            }
                        );
                    }
                }
                commands.entity(*e).insert(
                    SpriteSheetBundle {
                        transform: Transform {
                            scale: Vec3::splat(8.0),
                            translation: Vec3{x: position.x as f32 * 64., y: position.y as f32 * 64., z: 0.},
                            ..default()
                        },
                        texture: tileset.texture.clone(), // ????
                        atlas: TextureAtlas {
                            index: tile_id,
                            layout: tileset.layout.clone(), // ????
                        },
                        ..default()
                    }
                );
                return true;
            }
        }

        
        
        
        //let sprite = entity_ref.get::<Sprite>().unwrap();
    }   
}


