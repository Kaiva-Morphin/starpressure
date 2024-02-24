use bevy::prelude::*;

pub const DEFAULT_D: f32 = 100.; // todo: may change idk

#[derive(Component)]
pub struct Tile {
    pub d: f32,
    pub pos: [u32; 2]
}

impl Tile {
    pub fn default() -> Self {
        Self {
            d: 0.0,
            pos: [0, 0]
        }
    }
    pub fn new(pos: [u32; 2], d: f32) -> Self {
        Self {
            d,
            pos,
        }
    }
}

pub struct Neighbours {
    pub top: Option<Entity>,
    pub bottom: Option<Entity>,
    pub right: Option<Entity>,
    pub left: Option<Entity>,
}

impl Neighbours {
    pub fn default() -> Self {
        Self {
            top: None,
            bottom: None,
            right: None,
            left: None,
        }
    }
    pub fn is_all_none(&self) -> bool {
        self.top.is_none() && self.bottom.is_none() && self.right.is_none() && self.left.is_none()
    }
}

#[derive(Component)]
pub struct Wall {
    pub neighbours: Neighbours, // the tiles/walls that are connected to this one
    pub hp: u8,
    pub durability: u32,
    pub size: [u32; 2] // size in tiles
}

impl Wall {
    pub fn new(neighbours: Neighbours, hp: u8, durability: u32, size: [u32; 2]) -> Option<Self> {
        if neighbours.is_all_none() {
            return None;
        }
        
        Option::from(Self {
            neighbours,
            hp,
            durability,
            size,
        })
    }
}

#[derive(Component)]
pub struct Depressurized {
    pub data: bool
}

#[derive(Component)]
pub struct Simulate {
    pub data: bool
}

/*
#[derive(Component)]
pub struct Room {
    mask: Vec<Option<Entity>>,
    pub size: [u32; 2],
}

impl Room {
    pub fn new(size: [u32; 2]) -> Self {
        let mut mask = Vec::with_capacity((size[0] * size[1]) as usize);
        for _y in 0..size[1] {
            for _x in 0..size[0] {
                mask.push(None);
            }
        }
        Room {
            mask,
            size,
            depressurized: false,
        }
    }
    pub fn insert(&mut self, x: u32, y: u32, v: Option<Entity>) {
        // todo: add checks
        self.mask[(y * self.size[0] + x) as usize] = v;
    }
    pub fn get(&self, x: u32, y: u32) -> Option<Entity> {
        self.mask[(y * self.size[0] + x) as usize]
    }
}
*/
#[derive(Component)]
pub struct Room {
    // actual size in meters
    pub size_x: f32,
    pub size_y: f32,
}

#[derive(Component)]
pub struct BinMask {
    mask: Vec<bool>,
    pub size: [u32; 2],
}

impl BinMask {
    pub fn new(size: [u32; 2]) -> Self {
        let mut mask = Vec::with_capacity((size[0] * size[1]) as usize);
        for _y in 0..size[1] {
            for _x in 0..size[0] {
                mask.push(false);
            }
        }
        BinMask {
            mask,
            size,
        }
    }
    pub fn insert(&mut self, x: u32, y: u32, v: bool) {
        // todo: add checks
        self.mask[(y * self.size[0] + x) as usize] = v;
    }
    pub fn get(&self, x: u32, y: u32) -> bool {
        self.mask[(y * self.size[0] + x) as usize]
    }
}

#[derive(Component)]
pub struct DensityMask {
    mask: Vec<f32>,
    pub size: [u32; 2],
}

impl DensityMask {
    pub fn new(size: [u32; 2]) -> Self {
        let mut mask = Vec::with_capacity((size[0] * size[1]) as usize);
        for _y in 0..size[1] {
            for _x in 0..size[0] {
                mask.push(DEFAULT_D); 
            }
        }
        DensityMask {
            mask,
            size,
        }
    }
    pub fn insert(&mut self, x: u32, y: u32, v: f32) {
        // todo: add checks
        self.mask[(y * self.size[0] + x) as usize] = v;
    }
    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.mask[(y * self.size[0] + x) as usize]
    }
    pub fn iter(&self) -> std::slice::Iter<f32> {
        self.mask.iter()
    }
}

#[derive(Component, Debug)]
pub struct ForceMask {
    pub mask: Vec<(Vec2, f32)>, //todo: pub and Debug is for dbg, rm afterward
    pub size: [u32; 2],
}

impl ForceMask {
    pub fn new(size: [u32; 2]) -> Self {
        let mut mask = Vec::with_capacity((size[0] * size[1]) as usize);
        for _y in 0..size[1] {
            for _x in 0..size[0] {
                mask.push((Vec2::ZERO, 0.));
            }
        }
        ForceMask {
            mask,
            size,
        }
    }
    pub fn insert(&mut self, x: u32, y: u32, v: (Vec2, f32)) {
        // todo: add checks
        self.mask[(y * self.size[0] + x) as usize] = v;
    }
    pub fn get(&self, x: u32, y: u32) -> (Vec2, f32) {
        self.mask[(y * self.size[0] + x) as usize]
    }
}

