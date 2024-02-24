use bevy::prelude::*;
use bevy::ecs::schedule::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    InMenu,
    InGame,
}

pub fn menu_ph(
    mut appstate: ResMut<NextState<AppState>>
) {
    appstate.set(AppState::InGame);
}