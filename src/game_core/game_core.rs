use bevy::{ecs::{component::Component, entity::Entity, system::Resource}, transform::components::Transform, utils::hashbrown::HashMap};
use bevy_rapier2d::dynamics::Velocity;
use serde::{Deserialize, Serialize};



#[derive(Resource)]
pub struct GameManager{
    last_id: u64, 
    current_tick: u64,
    entity_from_id: HashMap<u64, Option<Entity>>
}
impl Default for GameManager{
    fn default() -> Self {
        GameManager{
            last_id: 0,
            current_tick: 0,
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
    pub fn tick_step(&mut self){
        self.current_tick += 1;
    }
    pub fn set_tick(&mut self, new_tick: u64){
        self.current_tick = new_tick;
    }
    pub fn get_tick(&self) -> u64 {
        return self.current_tick;
    }
}

#[derive(Serialize, Deserialize, Component)]
pub struct Object{
    id: u64,
}