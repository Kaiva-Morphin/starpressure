use bevy::prelude::*;





#[derive(Component)]
pub struct Object{
    id: u64
}

impl Object{
    pub fn new(id: u64) -> Self{
        Object { id }
    }
    pub fn id(&self) -> u64{
        return self.id
    }
}
