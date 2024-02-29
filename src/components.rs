use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorWorldPosition {
    pub pos: Vec2,
}

#[derive(Resource)]
pub struct CursorPosition {
    pub pos: Vec2,
}

#[derive(Resource)]
pub struct CursorEntity {
    pub entity: Option<Entity>,
}

#[derive(Resource)]
pub struct WindowSize {
    // physical size
    pub width: u32,
    pub height: u32,
}

#[derive(Component)]
pub struct Box;