use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::ragdoll::components::Joint;

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
pub struct Resizer { // unused; todo: check unused
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

#[derive(Event)]
pub struct JointSelectionOver;

#[derive(Event)]
pub struct CursorAboveUi;

#[derive(Component)]
pub struct RagdollTile {
    pub title: String,
    pub joints: HashMap<Entity, Joint>
}

impl RagdollTile {
    pub fn default() -> Self {
        RagdollTile { 
            title: "Jopa".to_owned(),
            joints: HashMap::new(),
        }
    }
}
#[derive(Clone)]
pub struct Selection {
    // ulrect and sgrect are the rects inside the atlas
    // sgrect is moved amd scaled with atlas, while ulrect is not
    pub ulrect: Rect,
    // unscaled local rect
    pub sgrect: Rect,
    // scaled global rect
    pub lpos: Vec2,
    // lpos is in unscaled, local coords, in the middle of rect
    pub gpos: Vec2,
    // gpos is in scaled, global coords, in the left bottom of rect
    pub entity: Entity,
    // entity of the selection
    pub joints: HashMap<Entity, Joint>,
    // joint and the entity its connected to
    pub parents: HashSet<Entity>,
}

impl Selection {
    pub fn empty() -> Self {
        Self {
            ulrect: Rect::default(),
            sgrect: Rect::default(),
            lpos: Vec2::ZERO,
            gpos: Vec2::ZERO,
            entity: Entity::PLACEHOLDER,
            joints: HashMap::new(),
            parents: HashSet::new(),
        }
    }
    pub fn new(ulrect: Rect, sgrect: Rect, lpos: Vec2, gpos: Vec2, entity: Entity) -> Self {
        Self {
            ulrect,
            sgrect,
            lpos,
            gpos,
            entity,
            joints: HashMap::new(),
            parents: HashSet::new(),
        }
    }
}

#[derive(Resource)]
pub struct AtlasData {
    pub name: String,
    pub scale: f32,
    pub rect: Rect,
    pub size: Vec2,
    pub loaded: bool,
    pub changed: bool,
    pub image: Handle<Image>,
    pub selected: Option<Entity>,
    pub pos: Vec2, // pos is in the left bottom corner of the sprite
    pub entity: Entity,
    pub selections: HashMap<Entity, Selection>,
    pub selected_joint1: Option<(Entity, Vec2, Vec2)>,
    pub selected_joint2: Option<(Entity, Vec2, Vec2)>,
}

impl AtlasData {
    pub fn default() -> Self {
        AtlasData {
            name: "".to_string(),
            scale: 1.,
            loaded: false,
            changed: false,
            size: Vec2::ZERO,
            rect: Rect::default(),
            entity: Entity::PLACEHOLDER,
            selected_joint1: None,
            selected_joint2: None,
            selections: HashMap::new(),
            pos: Vec2::ZERO,
            image: Handle::default(),
            selected: None,
        }
    }
}