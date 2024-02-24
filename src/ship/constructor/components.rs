use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ShipSave {
    pub tiles: Vec<Vec<[u32; 2]>>,
    pub walls: Vec<Vec<[u32; 2]>>,
    pub sizes: Vec<[u32; 2]>
}

impl ShipSave {
    pub fn new(n_rooms: usize) -> Self {
        let mut tiles = Vec::with_capacity(n_rooms);
        for _ in 0..n_rooms { tiles.push(vec![]) }
        let mut walls = Vec::with_capacity(n_rooms);
        for _ in 0..n_rooms { walls.push(vec![]) }
        let mut sizes = Vec::with_capacity(n_rooms);
        ShipSave {
            tiles,
            walls,
            sizes,
        }
    }
}