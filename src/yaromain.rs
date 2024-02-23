use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod ship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // states
        //.add_state::<AppState>()
        // events
        //.add_event::<UpdateMeshEvent>()
        // mod plugins
        .add_plugins((
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            ))
        
        .add_plugins(RapierDebugRenderPlugin::default())
        
        // own plugins
        //.add_plugins(())
        // systems
        
        .run()
}

