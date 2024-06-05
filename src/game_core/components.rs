use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct Name2Handle {
    data: HashMap<String, Handle<Image>>
}

impl Name2Handle {
    pub fn new() -> Self {
        Self { 
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, k: String, v: Handle<Image>) {
        self.data.insert(k, v);
    }
    
    pub fn get(&self, k: &str) -> Option<&Handle<Image>> {
        self.data.get(k)
    }
}
