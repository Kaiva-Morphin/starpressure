use bevy::prelude::*;

pub mod networking;

pub struct Networking;

impl Plugin for Networking {
    fn build(&self, app: &mut App) {
        //app
        //.add_event::<PlanetReshapeEvent>()
        //.insert_resource(Settings {freq: 1.5, ampl: 1.5, r: 5.})
        //.insert_resource(WorldGrid::new())
        //.add_systems(OnExit(GameState::MainMenu), t)
        //.add_systems(Update, (update_chunks.run_if(in_state(GameState::Game)), setup_shape, menu))
        //.add_systems(Update, (load_controller, unload_controller).run_if(in_state(GameState::Game)))
        //;
    }
}