use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_renet::{renet::{ConnectionConfig, RenetClient, RenetServer}, transport::{NetcodeClientPlugin, NetcodeServerPlugin}, RenetClientPlugin, RenetServerPlugin};



pub mod networking;
pub mod channels;
pub mod packets;
mod server;
mod client;

use client::*;
use renet_visualizer::{RenetClientVisualizer, RenetServerVisualizer};
use server::*;



pub struct Server;

impl Plugin for Server {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_server);
        app.add_systems(Update, (update_server, server_events));
        app.add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "server".into(),
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
        app.insert_resource(RenetServerVisualizer::<200>::default());     
        app.insert_resource(RenetServer::new(ConnectionConfig::default()));
    }
}

pub struct Client;

impl Plugin for Client {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_client);
        app.add_systems(Update, update_client);
        app.add_systems(FixedUpdate, send_message);
        app.add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "client".into(),
                ..default()
            }),
            ..default()
        }),
        EguiPlugin,
        WorldInspectorPlugin::new(),
        RenetClientPlugin,
        NetcodeClientPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin{enabled: false, ..default()}
        ));
        app.insert_resource(RenetClientVisualizer::<200>::default());     
        app.insert_resource(RenetClient::new(ConnectionConfig::default()));
    }
}