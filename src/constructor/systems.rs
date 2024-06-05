use bevy::{prelude::*, utils::{HashMap, HashSet}};
use bevy_file_dialog::prelude::*;

use crate::{
    components::CursorWorldPosition, consts::{TILE_SIZE, TILE_SIZE_I32}, editor::components::{CursorAboveUi, OpenFileEvent, SaveFileEvent}, ship::{
        components::{PlayerShip, Room, Tile, Wall, DEFAULT_D},
        rooms::{init_room, init_tile, init_wall}
    }, ShipFileContents
};

use super::components::{DrawBlueprint, PlaceData, Pos2Entity, RoomSave, SelectedTile, ShipSave, Tile4Save, TilesOrWalls, Wall4Save};

const MAX_ROOM_SIZE: usize = 1000;
const SIDES: [IVec2; 4] = [
    IVec2 {x: 0, y: 1},
    IVec2 {x: 0, y: -1},
    IVec2 {x: 1, y: 0},
    IVec2 {x: -1, y: 0},
];
pub const WALLS: [(&str, &str, Rect); 2] = [
    // editor name, atlas path name, rect
    ("Square", "wall.png", Rect { min: Vec2::ZERO, max: Vec2::splat(TILE_SIZE) }),
    ("test", "a.png", Rect { min: Vec2::ZERO, max: Vec2::splat(TILE_SIZE) }),
];

pub const TILES: [(&str, &str, Rect); 2] = [
    // editor name, atlas path name, rect
    ("red", "brokenwall.png", Rect { min: Vec2::ZERO, max: Vec2::splat(TILE_SIZE) }),
    ("test", "tiles.png", Rect { min: Vec2::ZERO, max: Vec2::splat(TILE_SIZE) }),
];

pub fn load_resources(
    mut commands: Commands
) {
    commands.insert_resource(Pos2Entity { data: HashMap::new() });
    commands.insert_resource(SelectedTile { hadle: None, rect: Rect::default() });
    commands.insert_resource(TilesOrWalls { is_tiles: false });
    commands.insert_resource(PlaceData { pos: IVec2::ZERO, destroy: None });
}

pub fn unload_resources(
    mut commands: Commands
) {
    commands.remove_resource::<Pos2Entity>();
    commands.remove_resource::<SelectedTile>();
    commands.remove_resource::<TilesOrWalls>();
    commands.remove_resource::<PlaceData>();
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

pub fn save_ship(
    mut commands: Commands,
    tiles_q: Query<&Tile4Save>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    pos2entity: Res<Pos2Entity>,
) {
    if keyboard_input.just_released(KeyCode::Equal) {
        let rooms = determine_rooms(&tiles_q, &pos2entity);
        for room in rooms {
            let mut roomsave = RoomSave::new();

        }
    }
}

pub fn select_rooms(
    mut commands: Commands,
    mut cursor_above_ui: EventReader<CursorAboveUi>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    pos2entity: &Res<Pos2Entity>,
    //mut firstpos: Local<Option<IVec2>>,
) {
    let mut aboveui = false;
    for _ in cursor_above_ui.read() { aboveui = true }
    if !aboveui && mouse_input.just_released(MouseButton::Middle) {
        //if let Some(entity) = pos2entity.data.get()
    }
}

// todo: next impl select rooms, because walls are hard to determine for a single room
// or separate walls and tiles
fn determine_rooms(
    tiles_q: &Query<&Tile4Save>,
    pos2entity: &Res<Pos2Entity>,
) -> Vec<HashSet<IVec2>> {
    let mut rooms = vec![];
    let mut starters = vec![];
    for starter in tiles_q.iter() {
        starters.push(starter.ipos);
        break;
    }
    let mut visited = HashSet::new();
    let mut every_visited = HashSet::new();
    loop {
        let mut new_starters = vec![];
        for starter in starters {
            if pos2entity.data.contains_key(&starter) && !visited.contains(&starter) {
                visited.insert(starter);
                for side in SIDES {
                    let new_ipos = starter + side;
                    new_starters.push(new_ipos);
                }
            }
        }
        if new_starters.is_empty() {
            let mut breakall = true;
            for tile in tiles_q.iter() {
                if !every_visited.contains(&tile.ipos) {
                    breakall = false;
                    new_starters.push(tile.ipos);
                    break;
                }
            }
            every_visited.extend(visited.clone());
            rooms.push(visited);
            visited= HashSet::new();
            if breakall {
                return rooms;
            }
        }
        starters = new_starters;
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
        .add_filter("Ship Binary", &["bin"])
        .load_file::<ShipFileContents>();
    }
    // save
    //for _ in save_file_event.read() {
    //    commands
    //    .dialog()
    //    .add_filter("Ship Binary", &["bin"])
    //    .set_file_name("ship.bin")
    //    .save_file::<ShipFileContents>(bincode::serialize(&save).unwrap());
    //}
}

pub fn draw_blueprint(
    mut commands: Commands,
    mut draw_blueprint_event: EventReader<DrawBlueprint>,
    mut blueprint_entity: Local<Option<Entity>>,
    selected: Res<SelectedTile>,
) {
    for blueprint in draw_blueprint_event.read() {
        if let Some(texture) = selected.hadle.clone() {
            let pos = blueprint.pos + TILE_SIZE / 2.;
            if let Some(entity) = *blueprint_entity {
                commands.entity(entity).insert(SpriteBundle {
                    texture,
                    transform: Transform::from_translation(pos),
                    sprite: Sprite {
                        rect: Some(selected.rect.clone()),
                        ..default()
                    },
                    ..default()
                });
            } else {
                let entity = commands.spawn(
                    SpriteBundle {
                        texture,
                        transform: Transform::from_translation(pos),
                        sprite: Sprite {
                            rect: Some(selected.rect.clone()),
                            ..default()
                        },
                        ..default()
                    },
                ).id();
                *blueprint_entity = Some(entity);
            }
        } else if let Some(entity) = *blueprint_entity {
            commands.entity(entity).despawn();
            *blueprint_entity = None;
        }
    }
}

pub fn place(
    mut commands: Commands,
    mut place_data: ResMut<PlaceData>,
    mut pos2entity: ResMut<Pos2Entity>,
    selected: Res<SelectedTile>,
    tilesorwalls: Res<TilesOrWalls>,
) {
    if let Some(destroy) = place_data.destroy {
        if let Some(texture) = selected.hadle.clone() {
            if let Some(entity) = pos2entity.data.get(&place_data.pos) {
                if destroy {
                    commands.entity(*entity).despawn();
                    pos2entity.data.remove(&place_data.pos);
                }
            } else if !destroy {
                let entity;
                if tilesorwalls.is_tiles {
                    entity = commands.spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform::from_translation((place_data.pos.as_vec2() * TILE_SIZE).extend(0.)  + TILE_SIZE / 2.),
                            sprite: Sprite {
                                rect: Some(selected.rect.clone()),
                                ..default()
                            },
                        ..default()
                        },
                        Tile4Save { ipos: place_data.pos },
                    )).id();
                } else {
                    entity = commands.spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform::from_translation((place_data.pos.as_vec2() * TILE_SIZE).extend(0.)  + TILE_SIZE / 2.),
                            sprite: Sprite {
                                rect: Some(selected.rect.clone()),
                                ..default()
                            },
                        ..default()
                        },
                        Wall4Save { ipos: place_data.pos },
                    )).id();
                }
                pos2entity.data.insert(place_data.pos, entity);
            }
        }
        place_data.destroy = None;
    }
}

pub fn process_selection(
    mouse_button: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorWorldPosition>,
    mut draw_blueprint_event: EventWriter<DrawBlueprint>,
    mut place_data: ResMut<PlaceData>,
) {
    let mut x = cursor_pos.pos.x as i32 / TILE_SIZE_I32;
    let mut y = cursor_pos.pos.y as i32 / TILE_SIZE_I32;
    if cursor_pos.pos.x < 0. {
        x -= 1
    }
    if cursor_pos.pos.y < 0. {
        y -= 1
    }
    let pos = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.);
    draw_blueprint_event.send(DrawBlueprint {
        pos,
    });
    let mut destroy = None;
    if mouse_button.just_released(MouseButton::Right) {
        destroy = Some(false);
    }
    
    if mouse_button.just_released(MouseButton::Left) {
        destroy = Some(true);
    }
    
    place_data.pos = IVec2::new(x, y);
    place_data.destroy = destroy;
}