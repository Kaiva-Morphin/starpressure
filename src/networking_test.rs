use bevy::prelude::*;

mod networking;
mod game_core;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};

use networking::*;
use game_core::*;

fn main(){
   let mut app = App::new();
    app.add_plugins((
        Networking,
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RUSTEROIDS server".into(),
                ..default()
            }),
            ..default()
        }),
        EguiPlugin,
        WorldInspectorPlugin::new(),
        RenetServerPlugin,
        NetcodeServerPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin{enabled: false, ..default()}
    ));
    app.run();
}
