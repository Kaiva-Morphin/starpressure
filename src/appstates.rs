use bevy::prelude::*;
use bevy::ecs::schedule::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    InMenu,
    InGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Game,
    Editor,
}

pub fn menu_ph(
    mut appstate: ResMut<NextState<AppState>>,
    mut gamestate: ResMut<NextState<GameState>>,
) {
    appstate.set(AppState::InGame);
    gamestate.set(GameState::Editor);
}