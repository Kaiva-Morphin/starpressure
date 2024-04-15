use bevy::prelude::*;
use bevy_file_dialog::prelude::*;

use crate::{
    components::CursorWorldPosition, consts::TILE_SIZE, editor::components::{OpenFileEvent, SaveFileEvent}, ship::{
        components::{PlayerShip, Room, Tile, Wall, DEFAULT_D},
        rooms::{init_room, init_tile, init_wall}
    }, ShipFileContents
};

use super::{components::{DrawBlueprint, RoomSave, ShipSave}, ConstructorState};

pub fn save_ship(
    ship_q: Query<&Children, With<PlayerShip>>,
    rooms_q: Query<&Children, With<Room>>,
    tiles_q: Query<&Tile>,
    walls_q: Query<&Wall>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::KeyS) {
        let rooms_entities = ship_q.single();
        let mut save = ShipSave::new(rooms_entities.len());
        for room_entity in rooms_entities.into_iter() {
            // here are all the room entities inside the ship
            let rooms_iter = rooms_q.get(*room_entity).into_iter();
            for children in rooms_iter {
                // here are all the rooms inside the ship
                let mut room_save = RoomSave::new();
                for child in children {
                    // here are all the tiles and walls inside a room
                    if let Ok(tile) = tiles_q.get(*child) {
                        room_save.tiles.push(tile.clone())
                    } else {
                        let wall = walls_q.get(*child).unwrap();
                        room_save.walls.push(wall.clone())
                    }
                }
                save.rooms.push(room_save);
            }
        }
        let file = std::fs::File::create("data.json").unwrap();
        //serde_json::to_writer(file, &save).unwrap();
    }
}

pub fn load_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    //if keyboard_input.just_released(KeyCode::KeyL) {
    //    let file = std::fs::File::open("data.json").unwrap();
    //    //let ship: ShipSave = serde_json::from_reader(file).unwrap();
    //    let mut room_entities = vec![];
    //    for room in ship.rooms {
    //        let mut children_entities = vec![];
    //        let room_entity = init_room(&mut commands, &asset_server, room.size);
    //        for tile in room.tiles {
    //            children_entities.push(init_tile(&mut commands, &asset_server, tile.pos, 10.));
    //        }
    //        for wall in room.walls {
    //            children_entities.push(init_wall(&mut commands, &asset_server, wall))
    //        }
    //        commands.entity(room_entity).push_children(&children_entities);
    //        room_entities.push(room_entity)
    //    }
    //    let ship = commands.spawn(PlayerShip)
    //    .insert((
    //        Name::new("ship"),
    //        TransformBundle::default(),
    //        VisibilityBundle::default()),)
    //    .id()
    //    ;
    //    commands.entity(ship).push_children(&room_entities);
    //}
}

pub fn init_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::KeyI) {
        let room = init_room(&mut commands, &asset_server, [2, 2]);
        let tile = init_tile(&mut commands, &asset_server, [0, 0], DEFAULT_D);
        commands.entity(room).add_child(tile);
        commands.spawn(PlayerShip)
        .insert((
            Name::new("ship"),
            TransformBundle::default(),
            VisibilityBundle::default(),))
        .add_child(room);
    }
}

pub fn place_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ship_q: Query<&Children, With<PlayerShip>>,
    rooms_q: Query<(&Room, &Children)>,
    cursor_pos: Res<CursorWorldPosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    // todo: remove from Update scedule, appstate mb
    if mouse_button.pressed(MouseButton::Left) {
        let selected_room_id = 0;
        let ship = ship_q.single();
        let selected_room_entity = ship[selected_room_id];
        let (room, children) = rooms_q.get(selected_room_entity).unwrap();
        
    }
}

pub fn dialog(
    mut commands: Commands,
    mut open_file_event: EventReader<OpenFileEvent>,
    mut save_file_event: EventReader<SaveFileEvent>,
    asset_server: Res<Assets<Image>>,
) {
    // open
    for _ in open_file_event.read() {
        commands
        .dialog()
        .add_filter("Ragdoll Binary", &["bin"])
        .load_file::<ShipFileContents>();
    }
    // save
    /*for _ in save_file_event.read() {
        commands
        .dialog()
        .add_filter("Ragdoll Binary", &["bin"])
        .set_file_name("ragdoll.bin")
        .save_file::<ShipFileContents>(bincode::serialize(&save).unwrap());
    }*/
}

pub fn draw_blueprint(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut draw_blueprint_event: EventReader<DrawBlueprint>,
    mut blueprint_entity: Local<Option<Entity>>,
) {
    for blueprint in draw_blueprint_event.read() {
        if let Some(entity) = *blueprint_entity {
            commands.entity(entity).insert(Transform::from_translation(blueprint.pos));
        } else {
            let entity = commands.spawn(
                SpriteBundle {
                    texture: asset_server.load("a.png"),
                    transform: Transform::from_translation(blueprint.pos),
                    sprite: Sprite {
                        rect: Some(blueprint.rect),
                        ..default()
                    },
                    ..default()
                },
            ).id();
            *blueprint_entity = Some(entity);
        }
    }
}

pub fn process_selection(
    mouse_button: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorWorldPosition>,
    constructor_state: Res<State<ConstructorState>>,
    mut draw_blueprint_event: EventWriter<DrawBlueprint>,
) {
    let selected_wall = 0.;
    match constructor_state.get() {
        ConstructorState::Walls => {
            draw_blueprint_event.send(DrawBlueprint { 
                pos: (cursor_pos.pos.floor() / TILE_SIZE).extend(0.),
                rect: Rect::from_corners(Vec2::ZERO, Vec2::new(20., 20.))
            });
        }
        ConstructorState::Tiles => {

        }
    }
}