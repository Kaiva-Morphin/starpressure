use bevy::prelude::*;

mod systems;

pub struct BackTilesPlugin;

impl Plugin for BackTilesPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_event::<PlanetReshapeEvent>()
        //.insert_resource(Settings {freq: 1.5, ampl: 1.5, r: 5.})
        //.add_systems(OnExit(GameState::MainMenu), t)
        //.add_systems(Update, (update_chunks.run_if(in_state(GameState::Game)), setup_shape, menu))
        //.add_systems(Update, (load_controller, unload_controller).run_if(in_state(GameState::Game)))
        ;
    }
}