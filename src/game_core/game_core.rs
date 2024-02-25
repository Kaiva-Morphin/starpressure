use bevy::{ecs::{component::Component, entity::Entity, system::Resource}, transform::components::Transform, utils::hashbrown::HashMap};
use bevy_rapier2d::dynamics::Velocity;
use serde::{Deserialize, Serialize};



#[derive(Resource)]
pub struct GameManager{
    last_id: u64, 
    entity_from_id: HashMap<u64, Option<Entity>>
}
impl Default for GameManager{
    fn default() -> Self {
        GameManager{
            last_id: 0,
            entity_from_id: HashMap::new(),
        }
    }
}
impl GameManager{
    pub fn new_id(&mut self) -> u64{
        self.last_id += 1;
        return self.last_id;
    }
    pub fn new_object(&mut self, entity: Entity) -> Object{
        let id = self.new_id();
        self.entity_from_id.insert(id, Some(entity));
        return Object{id};
    }
}

#[derive(Serialize, Deserialize, Component)]
pub struct Object{
    id: u64,
}