use std::{collections::HashSet, time::Instant};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::ship::components::{PlayerShip, Ship};
use crate::components::Box;
use super::components::{BinMask, DensityMask, Depressurized, ForceMask, Neighbours, Room, Simulate, Tile, Wall, DEFAULT_D};

pub const TILE_SIZE: f32 = 32.0; // in meters
pub const TILE_SIZE_USIZE: usize = TILE_SIZE as usize;
pub const TILE_SIZE_U32: u32 = TILE_SIZE as u32;

pub fn init_room (
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    size: [u32; 2],
) -> Entity {
    let binmask = BinMask::new(size);
    let densitymask = DensityMask::new(size);
    let forcemask = ForceMask::new(size);
    commands.spawn(binmask)
    .insert((
        densitymask,
        forcemask,
        Room { size_x: size[0] as f32 * TILE_SIZE, size_y: size[1] as f32 * TILE_SIZE },
        Depressurized { data: false },
        Simulate { data: true },
        Name::new("room"),
        TransformBundle::default(),
        VisibilityBundle::default(),))
    .id()
}

pub fn init_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: [u32; 2],
    d: f32,
) -> Entity{
    let tile = Tile::new(pos, d);
    let entity = commands.spawn(SpriteBundle {
        texture: asset_server.load("tiles.png"),
        transform: Transform::from_xyz(TILE_SIZE * pos[0] as f32, TILE_SIZE * pos[1] as f32, 0.),
        ..default()
    })
    .insert((
        Name::new(format!("Tile {:?}", tile.pos)),
        tile,))
    .id()
    ;
    entity
}

pub fn init_wall(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    wall: Wall,
) -> Entity {
    let entity = commands.spawn(
    SpriteBundle {
        texture: asset_server.load("wall.png"),
        transform: Transform::from_xyz(wall.pos[0] as f32, wall.pos[1] as f32, 0.),
        ..default()
        }
    )
    .insert((
        Wall::new(wall.neighbours, 100, 10, [1, 1]).unwrap(),
        Name::new("Wall"),
        Collider::cuboid(TILE_SIZE / 2., TILE_SIZE / 2.),
    ))
    .id()
    ;
    entity
}

pub fn process_air_leak(
    mut rooms_q: Query<(&mut DensityMask, &mut ForceMask, &BinMask)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::KeyP) {
        update_air_flow(&mut rooms_q, vec![[0, 0]]);
    }
}

pub fn update_air_flow(
    rooms_q: &mut Query<(&mut DensityMask, &mut ForceMask, &BinMask)>,
    starters: Vec<[u32; 2]>,
) -> bool {
    // todo: the vec2 part will stay the same, add extra func only for first time vec2 calc
    // otherwise do only f recalc, so save graph
    let air_movement_multipliter = 0.3; // todo: make it a const
    let (mut density_mask, mut force_mask, bin_mask) = rooms_q.single_mut();
    for i in starters.iter() {
        density_mask.insert(i[0], i[1], 0.)
    }
    let mut visited = HashSet::new();
    let n_starters = starters.len();
    let mut starters = Vec::from([starters]);
    let (size_x, size_y) = (bin_mask.size[0], bin_mask.size[1]);
    loop {
        let mut new_starters: Vec<Vec<[u32; 2]>> = Vec::with_capacity(n_starters);
        for _ in 0..n_starters { new_starters.push(vec![]) }
        for (starter_group_id, starters_group) in starters.into_iter().enumerate() {
            for starter in starters_group {
                if !visited.contains(&starter) {
                    visited.insert(starter);
                    let prev_density = density_mask.get(starter[0], starter[1]);
                    let new_x = starter[0] + 1;
                    if new_x < size_x {
                        if bin_mask.get(new_x, starter[1]) {
                            new_starters[starter_group_id].push([new_x, starter[1]]);
                            let own_density = density_mask.get(new_x, starter[1]);
                            let d_density = (prev_density - own_density).abs();
                            if d_density > 0.5 {
                                density_mask.insert(new_x, starter[1], (own_density - d_density * air_movement_multipliter).clamp(0., f32::MAX));
                                force_mask.insert(new_x, starter[1], (Vec2::new(1., 0.), d_density));
                            }
                        }
                    }
                    if starter[0] > 0 {
                        let new_x = starter[0] - 1;
                        if bin_mask.get(new_x, starter[1]) {
                            new_starters[starter_group_id].push([new_x, starter[1]]);
                            let own_density = density_mask.get(new_x, starter[1]);
                            let d_density = (prev_density - own_density).abs();
                            if d_density > 0.5 {
                                density_mask.insert(new_x, starter[1], (own_density - d_density * air_movement_multipliter).clamp(0., f32::MAX));
                                force_mask.insert(new_x, starter[1], (Vec2::new(-1., 0.), d_density));
                            }
                        }
                    }
                    if starter[1] + 1 < size_y {
                        let new_y = starter[1] + 1;
                        if bin_mask.get(starter[0], new_y) {
                            new_starters[starter_group_id].push([starter[0], new_y]);
                            let own_density = density_mask.get(starter[0], new_y);
                            let d_density = (prev_density - own_density).abs();
                            if d_density > 0.5 {
                                density_mask.insert(starter[0], new_y, (own_density - d_density * air_movement_multipliter).clamp(0., f32::MAX));
                                force_mask.insert(starter[0], new_y, (Vec2::new(0., 1.), d_density));
                            }
                        }
                        
                    }
                    if starter[1] > 0 {
                        let new_y = starter[1] - 1;
                        if bin_mask.get(starter[0], new_y) {
                            if bin_mask.get(starter[0], new_y) {
                                new_starters[starter_group_id].push([starter[0], new_y]);
                                let own_density = density_mask.get(starter[0], new_y);
                                let d_density = (prev_density - own_density).abs();
                                if d_density > 0.5 {
                                    density_mask.insert(starter[0], new_y, (own_density - d_density * air_movement_multipliter).clamp(0., f32::MAX));
                                    force_mask.insert(starter[0], new_y, (Vec2::new(0., -1.), d_density));
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut empty = true;
        for starters_group in new_starters.iter() {
            if starters_group.len() > 0 {
                empty = false;
                break;
            }
        }
        
        if empty { // todo: stop here is also for overall stop of leak update
            let mut stop = true;
            for i in density_mask.iter() {
                if i > &2. {
                    stop = false;
                    break;
                }
            }
            return stop;
        }
        starters = new_starters;
    }
}

pub fn apply_air_force(
    force_mask_q: Query<(&Room, &mut ForceMask, &Transform), Without<Box>>,
    mut rigid_body_q: Query<&mut Transform, With<Box>>,
) {
    if let Ok((room, mask, room_transform)) = force_mask_q.get_single() {
        let (room_pos_x, room_pos_y) = (room_transform.translation.x, room_transform.translation.y);
    
        for mut transform in rigid_body_q.iter_mut() {
            let (pos_x, pos_y) = (transform.translation.x, transform.translation.y);
            let rel_pos_x = (pos_x + room_pos_x) / TILE_SIZE;
            let rel_pos_y = (pos_y + room_pos_y) / TILE_SIZE;
            let (upos_x, upos_y) = (rel_pos_x as u32, rel_pos_y as u32);
            //println!("rel {} {}", rel_pos_x, rel_pos_y);
            let (v, f) = mask.get(upos_x, upos_y);
            let df = (v * f) / 100.; // m!!
            //println!("df {:?}", df);
            transform.translation += Vec3::new(df.x, df.y, 0.);
        }
    }
}

pub fn paint_walls(
    mut q: Query<(&mut Sprite, &Tile)>,
    mask: Query<&DensityMask>
) {
    if let Ok(mask) = mask.get_single() {
        for (mut sprite, tile) in q.iter_mut() {
            let pos = tile.pos;
            let d = mask.get(pos[0], pos[1]);
            let col = d / DEFAULT_D;
            sprite.color = Color::rgb(col, col, col);
        }
    }
}