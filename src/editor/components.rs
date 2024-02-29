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
pub struct TopNode;

#[derive(Component)]
pub struct FileTabNode {
    pub text_entity: Entity,
    pub image_entity: Entity,
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

#[derive(Serialize, Deserialize)]
pub struct RagdollSave {

} // to be used

#[derive(Component)] // unused
pub struct DynamicText {
    pub tl: Vec2, // top left
    pub br: Vec2, // bottom right
}

impl DynamicText {
    pub fn is_inside(&self, point: Vec2) -> bool {
        if self.tl.x >= point.x && self.tl.y >= point.y && point.x <= self.br.x && point.y <= self.br.y {
            return true;
        }
        return false;
    }
}
