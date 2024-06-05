use bevy::{prelude::*, utils::HashMap};
use serde::{Serialize, Deserialize};

use crate::ship::components::{Tile, Wall};

#[derive(Serialize, Deserialize)]
pub struct RoomSave {
    pub tiles: Vec<Tile>,
    pub walls: Vec<Wall>,
    pub size: [u32; 2],
}

impl RoomSave {
    pub fn new() -> Self {
        RoomSave {
            tiles: vec![],
            walls: vec![],
            size: [0, 0],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShipSave {
    pub rooms: Vec<RoomSave>,
}

impl ShipSave {
    pub fn new(n_rooms: usize) -> Self {
        let mut rooms = Vec::with_capacity(n_rooms);
        for _ in 0..n_rooms { rooms.push(RoomSave::new()) }
        ShipSave {
            rooms,
        }
    }
}

#[derive(Event)]
pub struct DrawBlueprint {
    pub pos: Vec3,
}

#[derive(Resource)]
pub struct PlaceData {
    pub pos: IVec2,
    pub destroy: Option<bool>,
}

#[derive(Resource)]
pub struct Pos2Entity {
    pub data: HashMap<IVec2, Entity>
}

#[derive(Component)]
pub struct SelectionSquare {
    pub handle: Handle<Image>,
    pub rect: Rect,
}

#[derive(Resource)]
pub struct SelectedTile {
    pub hadle: Option<Handle<Image>>,
    pub rect: Rect,
}

#[derive(Component)]
pub struct TilesButton;

#[derive(Component)]
pub struct WallsButton;

#[derive(Resource)]
pub struct TilesOrWalls {
    pub is_tiles: bool,
}

#[derive(Component)]
pub struct Tile4Save {
    pub ipos: IVec2,
}

#[derive(Component)]
pub struct Wall4Save {
    pub ipos: IVec2,
}