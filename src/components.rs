use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorPosition {
    pub pos: Vec2,
}

#[derive(Resource)]
pub struct CursorEntity {
    pub entity: Option<Entity>,
}