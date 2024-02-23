use bevy::prelude::*;

const TILE_DENSITY: f32 = 1.0;

struct Tile {
    density: f32,
    position: [u32; 2]
}

struct Room {
    tiles: Vec<Tile>,
}

pub fn init_room (

) {

}

pub fn init_tile(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("C:/Users/yaro4/Desktop/stuff/profile pic/arbrk.jpg"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}