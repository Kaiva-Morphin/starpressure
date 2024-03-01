use appstates::{menu_ph, AppState, GameState};
use bevy::prelude::*;
use bevy_file_dialog::prelude::*;
use bevy_rapier2d::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use components::{CursorEntity, CursorPosition, CursorWorldPosition, WindowSize};
use editor::EditorPlugin;
use ragdoll::RagdollPlugin;
use ship::ShipPlugin;

mod ship;
mod ragdoll;
mod editor;
mod appstates;
mod systems;
mod consts;
mod world_inspector;
pub mod components;

use systems::*;
use world_inspector::WorldInspectorPlugin;

struct RagdollFileContents;
//struct ShipFileContents;
struct AtlasFileContents;

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
        .init_state::<GameState>()
        // events
        //.add_event::<UpdateMeshEvent>()
        // resources
        .insert_resource(CursorEntity { entity: None })
        .insert_resource(CursorWorldPosition { pos: Vec2::ZERO })
        .insert_resource(CursorPosition { pos: Vec2::ZERO })
        .insert_resource(WindowSize {width: 1920, height: 1080})
        // mod plugins
        .add_plugins((
            bevy_egui::EguiPlugin,
            WorldInspectorPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            FileDialogPlugin::new()
                .with_save_file::<RagdollFileContents>()
                .with_load_file::<RagdollFileContents>()
                .with_load_file::<AtlasFileContents>(),
            ))
        .add_plugins(RapierDebugRenderPlugin::default())
        // own plugins
        .add_plugins((
            ShipPlugin,
            RagdollPlugin,
            EditorPlugin,
        ))
        // systems
        .add_systems(Startup, set_window_size)

        .add_systems(Update, 
            menu_ph
        .run_if(in_state(AppState::InMenu)))
        
        .add_systems(Update, (
            raycast, free_camera_controller
        ).run_if(in_state(AppState::InGame)))

        .add_systems(OnEnter(AppState::InGame), 
        (spawn_camera, spawn_floor).chain())
        .run()
}