use serde::{Deserialize, Serialize};
use crate::{networking::networking::ObjectData, InputKeys};




#[derive(Serialize, Deserialize)]
pub enum ServerDataPacket{
    Update{data: Vec<ObjectData>, tick: u64},
    Echo{time:f32},
}

#[derive(Serialize, Deserialize)]
pub enum ServerGaranteedDataPacket{
    Connected,
    Message{text: String},
}


#[derive(Serialize, Deserialize)]
pub enum ClientDataPacket{
    Inputs{keys: InputKeys},
    Echo{time:f32},
}

#[derive(Serialize, Deserialize)]
pub enum ClientGaranteedDataPacket{
    Message{text: String},
}