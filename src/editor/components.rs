use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component)]
pub struct SelectionTab {
    pub size: f32,
}

#[derive(Component)]
pub struct FileButton {
    pub is_opened: bool,
}

#[derive(Component)]
pub struct OpenFileButton;

#[derive(Component)]
pub struct NewFileButton;

#[derive(Component)]
pub struct SaveFileButton;

#[derive(Component)]
pub struct LoadAtlasButton;

#[derive(Component)]
pub struct TopNode;

#[derive(Component)]
pub struct Arrow;

#[derive(Component)]
pub struct FileTabNode {
    pub text_entity: Entity,
    pub image_entity: Entity,
    pub is_opened: bool,
}

#[derive(Component)]
pub struct FileTab {
    pub top_entity: Entity,
}

#[derive(Component)]
pub struct Resizer {
    pub was_pressed: bool,
    pub is_vertical: bool,
    pub start_pos: f32,
}

#[derive(Component)]
pub struct Atlas;

#[derive(Event)]
pub struct ResizeEvent {
    pub new_pos: f32,
    pub entity: Entity,
    pub is_vertical: bool,
}

#[derive(Event)]
pub struct FileOpenWindowEvent {
    pub entity: Entity,
    pub to_open: bool,
}

#[derive(Event)]
pub struct NewFileEvent;

#[derive(Event)]
pub struct SaveFileEvent;

#[derive(Event)]
pub struct OpenFileEvent;

#[derive(Event)]
pub struct LoadAtlasEvent;

#[derive(Serialize, Deserialize)]
pub struct RagdollSave {

} // to be used

#[derive(Component)]
pub struct Tile {
    pub title: String,
}

impl Tile {
    pub fn default() -> Self {
        Tile { 
            title: "Jopa".to_owned()
        }
    }
}

#[derive(Resource)]
pub struct AtlasData {
    pub scale: f32,
    pub rect: Rect,
    pub size: Vec2,
    pub loaded: bool,
    pub changed: bool,
    pub image: Handle<Image>,
    pub selected: Option<usize>,
    pub pos: Vec2, // pos is in the middle of the sprite
    pub entity: Entity,
    pub selections: Vec<(Rect, Vec2, Entity)>,
}

impl AtlasData {
    pub fn default() -> Self {
        AtlasData { 
            scale: 1.,
            loaded: false,
            changed: false,
            size: Vec2::ZERO,
            rect: Rect::default(),
            entity: Entity::PLACEHOLDER,
            selections: vec![],
            pos: Vec2::ZERO,
            image: Handle::default(),
            selected: None,
        }
    }
}