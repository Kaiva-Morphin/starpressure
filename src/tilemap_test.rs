use bevy::{prelude::*, render::render_resource::Texture};

mod networking;
mod game_core;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{dynamics::RigidBody, geometry::Collider, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};

use networking::{networking::*, *};
use game_core::{game_core::GameManager, tilemap::*, *};




const SERVER_TPS: f64 = 1.; 

fn main(){
    let mut app = App::new();
    app.add_systems(Startup, (init, init_tiles).chain());
    app.add_systems(Update, update);
    app.add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "server".into(),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()),
        EguiPlugin,
        WorldInspectorPlugin::new(),
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin{enabled: false, ..default()}
    ));
    app.insert_resource(GameManager::default());
    app.insert_resource(Time::<Fixed>::from_seconds(1. / SERVER_TPS));
    app.run();
}

fn init(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    
){
    commands.spawn(Camera2dBundle::default());

    /*let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 2, 2, None, None);
    let texture_atlas_handle: Handle<TextureAtlasLayout> = texture_atlases.add(texture_atlas);
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            scale: Vec3::splat(10.0),
            ..default()
        },
        texture: texture_handle,
        atlas: TextureAtlas {
            index: 7,
            layout: texture_atlas_handle,
        },
        ..default()
    });*/
    let mut collection = TileSetCollection::init(&mut texture_atlases, &asset_server);
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(8.0, 8.0), 17, 1, None, None);
    let texture_atlas_handle: Handle<TextureAtlasLayout> = texture_atlases.add(texture_atlas);

    let tileset_id = collection.register_new_tileset(texture_atlas_handle, texture_handle);
    collection.set_tiles(
        tileset_id,
        vec![
            Tile::Singletile,

            Tile::MultitileOrigin{parts_offsets_ids: vec![(IVec2{x: 1, y: 0}, 2)]},
            Tile::Multitile{origin_offset: IVec2{x: -1, y: 0}, origin_id: 1},

            Tile::MultitileOrigin{parts_offsets_ids: vec![(IVec2{x: 0, y: -1}, 4)]},
            Tile::Multitile{origin_offset: IVec2{x: 0, y: 1}, origin_id: 3},

            Tile::MultitileOrigin{parts_offsets_ids: vec![(IVec2{x: 1, y: 0}, 6)]},
            Tile::Multitile{origin_offset: IVec2{x: -1, y: 0}, origin_id: 5},

            Tile::MultitileOrigin{parts_offsets_ids: vec![(IVec2{x: 1, y: 0}, 8)]},
            Tile::Multitile{origin_offset: IVec2{x: -1, y: 0}, origin_id: 7},

            Tile::Singletile,
            Tile::Singletile,
            Tile::Singletile,
            Tile::Singletile,

            Tile::Multitile{origin_offset: IVec2{x: 0, y: -1}, origin_id: 15},
            Tile::Multitile{origin_offset: IVec2{x: -1, y: -1}, origin_id: 15},
            Tile::MultitileOrigin{parts_offsets_ids: vec![(IVec2{x: 0, y: 1}, 13), (IVec2{x: 1, y: 1}, 14), (IVec2{x: 1, y: 0}, 16)]},
            Tile::Multitile{origin_offset: IVec2{x: -1, y: 0}, origin_id: 15},
        ]
    );

    

    commands.spawn((
        Collider::cuboid(100., 10.),
        RigidBody::Fixed,
    )).insert(GlobalTransform::from(Transform::from_translation(Vec3{x: 0., y:-200., z:0.})));

    let e = commands.spawn((
        Name::from(String::from("SHIP")),
        TransformBundle::default(),
        VisibilityBundle::default(),
        Ship,
    )).id();

    TileMap::init_for(e, UVec2{x: 10, y: 10}, &mut commands, &collection); // todo: switch to bulder!

    let e = commands.spawn((
        Name::from(String::from("SHIP2")),
        TransformBundle::default(),
        VisibilityBundle::default(),
        Ship,
    )).id();

    TileMap::init_for(e, UVec2{x: 5, y: 8}, &mut commands, &collection); // todo: switch to bulder!
    commands.entity(e).insert(Transform::from_translation(Vec3{x: -100., y: -10., z: 0.}).with_rotation(Quat::from_euler(EulerRot::XYZ, 0., 0., 3.14 / 3.)));

    commands.insert_resource(collection);
}

#[derive(Component)]
struct  Ship;

fn init_tiles(
    mut ship_q: Query<&mut TileMap, With<Ship>>,
    mut commands: Commands,
    collection: Res<TileSetCollection>
){
    let mut i = 0;
    for mut tilemap in ship_q.iter_mut(){
        i += 1;
        if i == 1{
            let tileset_id = 1;
            let mut pasted_position_id = 0;
            for i in 0..collection.tilesets.get(tileset_id).unwrap().tiles.len(){
                let is_pasted = tilemap.set_tile(
                    &mut commands,
                    UVec2 { x: (pasted_position_id * 2) % 10, y: (pasted_position_id * 2 / 10) * 2 + 1 },
                    &collection,
                    tileset_id,
                    i
                );
                if is_pasted {pasted_position_id += 1;}
            }
        } else {
            let mut set_tile = |x: u32, y: u32, tile_id: usize|{
                tilemap.set_tile(
                    &mut commands,
                    UVec2 { x, y },
                    &collection,
                    1,
                    tile_id
                );
            };
            set_tile(0, 0, 15);
            set_tile(3, 0, 15);
            set_tile(2, 2, 3);
            set_tile(2, 3, 0);
            set_tile(2, 4, 0);
            set_tile(2, 5, 0);
            set_tile(2, 6, 9);
            set_tile(1, 6, 10);
            set_tile(1, 5, 12);
        }
    }

}



fn update(
    mut ship_q: Query<(&mut TileMap, &Transform), With<Ship>>,
    mut gizmos: Gizmos,
    mut commands: Commands,
){
    for (mut tilemap, transform) in ship_q.iter_mut(){
        tilemap.draw_grid(&mut gizmos, &transform);
    }
}

