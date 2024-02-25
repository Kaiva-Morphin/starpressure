use appstates::{menu_ph, AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::{CursorEntity, CursorPosition, WindowSize};
use ship::ShipPlugin;
mod ship;
mod appstates;
mod systems;
pub mod components;

use systems::*;


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
        // resources
        .insert_resource(CursorEntity { entity: None })
        .insert_resource(CursorPosition { pos: Vec2::ZERO })
        .insert_resource(WindowSize {width: 1920, height: 1080})
        // mod plugins
        .add_plugins((
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            ))
        .add_plugins(RapierDebugRenderPlugin::default())
        // own plugins
        .add_plugins(ShipPlugin)
        // systems
        .add_systems(Startup, set_window_size)

        .add_systems(Update, 
            menu_ph
        .run_if(in_state(AppState::InMenu)))
        
        .add_systems(Update, (
            raycast, draw_net
        ).run_if(in_state(AppState::InGame)))

        .add_systems(OnEnter(AppState::InGame), (spawn_camera, spawn_box).chain())
        .run()
}