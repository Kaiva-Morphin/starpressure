use std::{thread, time::Duration};

use bevy::{prelude::*};

mod networking;
mod game_core;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{dynamics::{FixedJoint, GravityScale, RigidBody, Sleeping, Velocity}, geometry::Collider, plugin::{NoUserData, RapierPhysicsPlugin}, rapier::{dynamics::{BodyPair, RigidBodyHandle}, geometry::{ColliderBuilder, ColliderSet}}, render::RapierDebugRenderPlugin};
use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};


use networking::{networking::*, *};
use game_core::*;




const SERVER_TPS: f64 = 1.; 

fn main(){
    let mut app = App::new();
    app.add_plugins((
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
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0), // ::<NoUserData>::pixels_per_meter(15.0)
        RapierDebugRenderPlugin{enabled: true, ..default()}
    ));
    app.add_systems(Startup, init);
    app.add_systems(Update, update);
    app.run();
}




fn init(
    mut commands: Commands,
    
){
    commands.spawn((Camera2dBundle::default(),
    bevy_rapier2d::prelude::CollisionGroups::default()
    ));
    
}



fn update(
    mut gizmos: Gizmos,
){
    gizmos.line_2d(Vec2::Y , Vec2::splat(80.), Color::RED);
}

