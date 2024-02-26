use bevy::math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct ObjectData{
    pub linvel: Vec2,
    pub position: Vec3,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InputKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}






