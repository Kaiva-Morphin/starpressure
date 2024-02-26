use bevy::{asset::{AssetServer, Assets, Handle}, core::Name, ecs::{bundle::Bundle, component::Component, entity::Entity, system::{Commands, Res, ResMut, Resource}, world::World}, gizmos::gizmos::Gizmos, hierarchy::{BuildChildren, ChildBuilder}, log::warn, math::{IVec2, UVec2, Vec2, Vec3, Vec4}, prelude::default, render::{color::Color, render_asset::RenderAssetUsages, render_resource::{Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, texture::{CompressedImageFormats, Image, ImageSampler, ImageType}}, sprite::{Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, transform::components::Transform, utils::hashbrown::HashMap};

/*
pub struct TileCollisionShapes{
    
}
*/




// todo: this enum for test only, add multitile generator and switch to struct! (this may be used as tile_type)
// generator must put multitile pieces in end of vec, out of base len.
#[allow(dead_code)]
#[derive(Clone)]
pub enum Tile{ 
    Singletile,
    MultitileOrigin{parts_offsets_ids: Vec<(IVec2, usize)>},
    Multitile{origin_offset: IVec2, origin_id: usize}
}

/*
pub struct Tile {
    durability: ?,
    airflow: f32, // 0 -> 1, ability of tile to pass some air
    waterflow: f32,
    collision_shape: ???, // id -> shape in atlas
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

#[allow(dead_code)]
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

#[derive(Component)]
pub struct TileMap{ // todo: tilemap ship bundle!
    size: UVec2, // left down is 0, 0 corner
    tile_entities: Vec<Vec<Entity>>, 
    tile_ids: Vec<Vec<(usize, usize)>>,

    //bg_tiles: Vec<Vec<usize>>,
}

impl TileMap{
    pub fn init_for(e: Entity, size: UVec2, commands: &mut Commands, collection: &TileSetCollection){ // todo: switch to bulder!
        let mut tile_entities = vec![];
        let mut tile_ids = vec![];
        commands.entity(e).with_children(|children_builder|{
            let air_tileset = collection.tilesets.get(0).unwrap();
            for x in 0..size.x{
                let mut entity_row = vec![];
                let mut tile_row = vec![];
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
                    entity_row.push(e);
                    tile_row.push((0, 0));
                }
                tile_entities.push(entity_row);
                tile_ids.push(tile_row)
            }
        }).insert(TileMap{size, tile_entities, tile_ids});
    }
    pub fn size(&self) -> UVec2{
        return self.size;
    }
    pub fn draw_grid(
        &self,
        gizmos: &mut Gizmos,
        transform: &Transform
    ){

        for x in 0..=self.size.x{
            let matrix = transform.compute_matrix();
            gizmos.line_2d(
                (matrix.transform_point3(Vec3{x: x as f32 * 64. - 32., y: 0. - 32., z: 0.})).truncate(),
                (matrix.transform_point3(Vec3{x: x as f32 * 64. - 32., y: self.size.y as f32 * 64. - 32., z: 0.})).truncate(),
                Color::BLACK
            );
            for y in 0..=self.size.y{
                gizmos.line_2d(
                    (matrix.transform_point3(Vec3{x: 0. - 32., y: y as f32 * 64. - 32., z: 0.})).truncate(),
                    (matrix.transform_point3(Vec3{x: self.size.x as f32 * 64. - 32., y: y as f32 * 64. - 32., z: 0.})).truncate(),
                    Color::BLACK
                );
            }
        }
    }

    pub fn set_tile(
        &mut self,
        commands: &mut Commands,
        position: UVec2, // todo: swap to x, y?
        collection: &TileSetCollection,
        tileset_id: usize,
        tile_id: usize
    ) -> bool {
        let tileset = collection.tilesets.get(tileset_id).unwrap();
        let tile = collection.get_tile(tileset_id, tile_id).unwrap();
        //println!("{}", position );

        let e = if let Some(row) = self.tile_entities.get(position.x as usize){
            row.get(position.y as usize).unwrap().clone()
        } else {
            warn!("something wrong!");
            return false;
        };

        match tile{
            Tile::Singletile => {
                if let Some(tile) = self.get_tile(position) {if *tile != (0, 0) {return false}};
                self.set_texture(commands, e, (position.x as f32 * 64., position.y as f32 * 64.), 0, tileset);
                self.tile_ids.get_mut(position.x as usize).unwrap().insert(position.y as usize, (tileset_id, tile_id));
                return true;
            }
            Tile::Multitile { origin_offset: _, origin_id: _ } => {
                // todo: ignore?
                return false;
            }
            Tile::MultitileOrigin { parts_offsets_ids } => {
                for (offset, id) in parts_offsets_ids.iter(){
                    let part_pos = IVec2{x: position.x as i32 + offset.x, y: position.y as i32 + offset.y};
                    //if let Some(tile) = self.get_tile(UVec2 { x: part_pos.x as u32, y: part_pos.y as u32 }) {if *tile != (0, 0) {return false}};
                    if !self.in_bounds_i(&part_pos){return false}
                }

                for (offset, id) in parts_offsets_ids.iter(){
                    let part_pos = IVec2{x: position.x as i32 + offset.x, y: position.y as i32 + offset.y};
                    if self.in_bounds_i(&part_pos){
                        let e = self.tile_entities.get(part_pos.x as usize).unwrap().get(part_pos.y as usize).unwrap().clone();
                        self.set_texture(commands, e, (part_pos.x as f32 * 64., part_pos.y as f32 * 64.), *id, tileset);
                        self.tile_ids.get_mut(part_pos.x as usize).unwrap().insert(part_pos.y as usize, (tileset_id, *id));
                    }
                }
                self.set_texture(commands, e, (position.x as f32 * 64., position.y as f32 * 64.), tile_id, tileset);
                self.tile_ids.get_mut(position.x as usize).unwrap().insert(position.y as usize, (tileset_id, tile_id));
                return true;
            }
        }
        //let sprite = entity_ref.get::<Sprite>().unwrap();
    }

    fn in_bounds(
        &self,
        position: &UVec2
    ) -> bool {
        return position.x < self.size.x as u32 && position.y < self.size.y as u32;
    }

    fn in_bounds_i(
        &self,
        position: &IVec2
    ) -> bool {
        return position.x >= 0 && position.y >= 0 && position.x < self.size.x as i32 && position.y < self.size.y as i32;
    }

    pub fn remove_tile(
        &mut self,
        position: UVec2,
        collection: &TileSetCollection,
        commands: &mut Commands
    ) -> bool {
        let e = self.tile_entities.get(position.x as usize).unwrap().get(position.y as usize).unwrap().clone();
        let tile_id = self.tile_ids.get(position.x as usize).unwrap().get(position.y as usize).unwrap();
        if *tile_id == (0, 0) {return false;}
        let tile = collection.get_tile(tile_id.0, tile_id.1).unwrap();
        let tileset = collection.tilesets.get(0).unwrap();
        match tile{
            Tile::Singletile => {
                self.set_texture(commands, e, (position.x as f32 * 64., position.y as f32 * 64.), 0, tileset);
                self.tile_ids.get_mut(position.x as usize).unwrap().insert(position.y as usize, (0, 0));
                return true;
            }
            Tile::Multitile { origin_offset, origin_id} => {
                let origin_pos = IVec2{x: position.x as i32 + origin_offset.x, y: position.y as i32 + origin_offset.y};
                if !self.in_bounds_i(&origin_pos){return false;};
                let origin_tile = collection.get_tile(1, *origin_id).unwrap(); // todo: add handle for othertilesets!
                match tile{
                    Tile::MultitileOrigin { parts_offsets_ids } => {
                        for (offset, id) in parts_offsets_ids.iter(){
                            let part_pos = IVec2{x: position.x as i32 + offset.x, y: position.y as i32 + offset.y};
                            if self.in_bounds_i(&part_pos){
                                let e = self.tile_entities.get(part_pos.x as usize).unwrap().get(part_pos.y as usize).unwrap().clone();
                                self.set_texture(commands, e, (part_pos.x as f32 * 64., part_pos.y as f32 * 64.), 0, tileset);
                                self.tile_ids.get_mut(part_pos.x as usize).unwrap().insert(part_pos.y as usize, (0, 0));
                            }
                        }
                        self.set_texture(commands, e, (position.x as f32 * 64., position.y as f32 * 64.), 0, tileset);
                        self.tile_ids.get_mut(position.x as usize).unwrap().insert(position.y as usize, (0, 0));
                    }
                    _ => {return false}
                }
                return true;
            }
            Tile::MultitileOrigin { parts_offsets_ids } => {
                for (offset, id) in parts_offsets_ids.iter(){
                    let part_pos = IVec2{x: position.x as i32 + offset.x, y: position.y as i32 + offset.y};
                    if self.in_bounds_i(&part_pos){
                        let e = self.tile_entities.get(part_pos.x as usize).unwrap().get(part_pos.y as usize).unwrap().clone();
                        self.set_texture(commands, e, (position.x as f32 * 64., position.y as f32 * 64.), 0, tileset);
                        self.tile_ids.get_mut(position.x as usize).unwrap().insert(position.y as usize, (0, 0));
                    }
                }
                self.set_texture(commands, e, (position.x as f32 * 64., position.y as f32 * 64.), 0, tileset);
                self.tile_ids.get_mut(position.x as usize).unwrap().insert(position.y as usize, (0, 0));
                return true;
            }
        }
    }

    fn set_texture(
        &self,
        commands: &mut Commands,
        entity: Entity,
        position: (f32, f32),
        index: usize,
        tileset: &TileSet
    ){
        commands.entity(entity).insert((
            tileset.texture.clone(),
            TextureAtlas {
                index: index,
                layout: tileset.layout.clone(), // ????
            }
        ));
    }

    pub fn get_tile(
        &self,
        position: UVec2
    ) -> Option<&(usize, usize)> {
        if let Some(row) = self.tile_ids.get(position.x as usize){
            return row.get(position.y as usize)
        }
        return None;
    }   
}


