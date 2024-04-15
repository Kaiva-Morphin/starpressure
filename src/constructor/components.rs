use bevy::prelude::*;
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
    pub rect: Rect,
}