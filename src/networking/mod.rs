use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::*, rapier::{dynamics::IntegrationParameters, pipeline::PhysicsPipeline}, render::RapierDebugRenderPlugin};
use bevy_renet::{renet::{ConnectionConfig, RenetClient, RenetServer}, transport::{NetcodeClientPlugin, NetcodeServerPlugin}, RenetClientPlugin, RenetServerPlugin};



pub mod networking;
pub mod channels;
pub mod packets;
mod server;
mod client;

use client::*;
use renet_visualizer::{RenetClientVisualizer, RenetServerVisualizer};
use server::*;

use crate::game_core::game_core::GameManager;


const SERVER_TPS : f64 = 5.;
pub struct Server;

impl Plugin for Server {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_server);
        app.add_systems(Update, update_visualizer);
        app.add_systems(FixedUpdate, (update_server, server_events)); // after rapeir physics
        app.insert_resource(RapierConfiguration{timestep_mode: {TimestepMode::Fixed { dt: 1. / (SERVER_TPS as f32 * 0.1), substeps: 1 }}, ..default()});
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
            RapierPhysicsPlugin::<NoUserData>::default().in_schedule(FixedPreUpdate),
            RapierDebugRenderPlugin{enabled: false, ..default()}
        ));
        
        app.insert_resource(GameManager::default());
        app.insert_resource(RenetServerVisualizer::<200>::default());     
        app.insert_resource(RenetServer::new(ConnectionConfig::default()));
        app.insert_resource(Time::<Fixed>::from_seconds(1. / SERVER_TPS));
    }
    
}

pub struct Client;

impl Plugin for Client {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_client);
        app.add_systems(Update, update_client_visualizer);
        app.add_systems(FixedUpdate, update_client);
        app.insert_resource(RapierConfiguration{timestep_mode: {TimestepMode::Fixed { dt: 1. / (SERVER_TPS as f32 * 5. * 0.1), substeps: 1 }}, ..default()});
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
        RapierPhysicsPlugin::<NoUserData>::default().in_schedule(FixedPreUpdate),
        RapierDebugRenderPlugin{enabled: false, ..default()}
        ));
        app.insert_resource(GameManager::default());
        app.insert_resource(RenetClientVisualizer::<200>::default());     
        app.insert_resource(RenetClient::new(ConnectionConfig::default()));
        app.insert_resource(Time::<Fixed>::from_seconds(1. / (SERVER_TPS * 5.)));
    }
}


