use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Bone;

#[derive(Component)]
pub struct Ragdoll; // ????

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Joint {
    // they are relative to center, they must be
    pub origin1: Vec2,
    pub origin2: Vec2,
    pub hs: Vec2,
    // todo: add min max rotation
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectionSave {
    pub ulrect: Rect,
    pub lpos: Vec2,
    pub entity: Entity,
    pub joints: HashMap<Entity, Joint>,
    pub parents: HashSet<Entity>,
}

impl SelectionSave {
    pub fn new(ulrect: Rect, lpos: Vec2, entity: Entity, joints: HashMap<Entity, Joint>, parents: HashSet<Entity>) -> Self {
        Self {
            ulrect,
            lpos,
            entity,
            joints,
            parents,
        }
    }
}

#[derive(Serialize, Deserialize, Event, Default, Debug)]
pub struct RagdollSave {
    pub name: String,
    pub saves: Vec<SelectionSave>,
}
