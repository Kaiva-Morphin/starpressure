use appstates::{menu_ph, AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ship::ShipPlugin;
mod ship;
mod appstates;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest()
            ).set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Starpressure".into(),
                    ..default()
                }),
                ..default()
            }),)
        // states
        .init_state::<AppState>()
        // events
        //.add_event::<UpdateMeshEvent>()
        // mod plugins
        .add_plugins((
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            ))
        
        .add_plugins(RapierDebugRenderPlugin::default())
        
        // own plugins
        .add_plugins(ShipPlugin)
        // systems
        .add_systems(Update, menu_ph.run_if(in_state(AppState::InMenu)))
        .run()
}