use bevy::{prelude::*, render::render_resource::Texture, window::PrimaryWindow};

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
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    collection: Res<TileSetCollection>
){
    if buttons.pressed(MouseButton::Left) || buttons.pressed(MouseButton::Right) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            for (mut tilemap, transform) in ship_q.iter_mut(){
                let ingrid_vec = world_position.extend(0.) - transform.translation;
                let unwrapped_world_vec = transform.rotation.inverse().mul_vec3(ingrid_vec);
                let size = tilemap.size() * 64;
                let size = Vec2::from([size.x as f32, size.y as f32]);

                let mut set_tile = |x: u32, y: u32, tileset_id: usize, tile_id: usize|{
                    tilemap.set_tile(
                        &mut commands,
                        UVec2 { x, y },
                        &collection,
                        tileset_id,
                        tile_id
                    );
                };
                
                
                if -32. < unwrapped_world_vec.x && -32. < unwrapped_world_vec.y && unwrapped_world_vec.x < size.x + 32. && unwrapped_world_vec.y < size.y + 32.{
                    let cell_vec = unwrapped_world_vec.truncate() + Vec2::splat(32.);
                    let cell = cell_vec / 64.;
                    if cell.x >= 0. && cell.y >= 0. && cell.x <= size.x && cell.y <= size.y{
                        let cell_pos = UVec2{x: cell.x as u32, y: cell.y as u32};
                        if buttons.pressed(MouseButton::Left){
                            set_tile(cell_pos.x, cell_pos.y, 1, 0);
                        } else {
                            tilemap.remove_tile(cell_pos, &collection, &mut commands); // 0 0 is air
                        }
                    } 
                }
            }
        }
    }
    for (mut tilemap, transform) in ship_q.iter_mut(){
        tilemap.draw_grid(&mut gizmos, &transform);
    }
}

