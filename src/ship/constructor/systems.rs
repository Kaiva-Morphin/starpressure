use bevy::prelude::*;
use serde_json::json;

use crate::ship::{components::PlayerShip, init_room, init_tile, tiles::components::{Room, Tile, Wall}};

use super::components::ShipSave;

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
        for (room_id, room_entity) in rooms_entities.into_iter().enumerate() {
            // here are all the room entities inside the ship
            let rooms_iter = rooms_q.get(*room_entity).into_iter();
            for children in rooms_iter {
                // here are all the rooms inside the ship
                for child in children {
                    // here are all the tiles and walls inside a room
                    if let Ok(tile) = tiles_q.get(*child) {
                        save.tiles[room_id].push(tile.pos)
                    } else {
                        let wall = walls_q.get(*child).unwrap();
                        save.walls[room_id].push(wall.pos)
                    }
                }
                
            }
        }
        let file = std::fs::File::create("data.json").unwrap();
        serde_json::to_writer(file, &save).unwrap();
    }
}

pub fn load_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::KeyL) {
        let file = std::fs::File::open("data.json").unwrap();
        let ship: ShipSave = serde_json::from_reader(file).unwrap();
        let mut room_entities = vec![];
        for room_id in 0..ship.sizes.len() {
            let mut children_entities = vec![];
            let room_size = ship.sizes[room_id];
            let room_tiles = &ship.tiles[room_id];
            let room_walls = &ship.walls[room_id];
            let room_entity = init_room(&mut commands, &asset_server, room_size);
            for tile_pos in room_tiles {
                children_entities.push(init_tile(&mut commands, &asset_server, *tile_pos, 10.));
            }
            // for wall_pos...
            commands.entity(room_entity).push_children(&children_entities);
            room_entities.push(room_entity)
        }
        let ship = commands.spawn(PlayerShip)
        .insert((
            Name::new("ship"),
            TransformBundle::default(),
            VisibilityBundle::default()),)
        .id()
        ;
        commands.entity(ship).push_children(&room_entities);
    }
}