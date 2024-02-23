use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct ObjectData{}

#[derive(Serialize, Deserialize)]
pub struct InputKeys {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}






