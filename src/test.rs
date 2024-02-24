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
    for i in 0..5 {
        async{
            thread::sleep(Duration::from_secs(1));
            println!("{}", i + 1);
        };
    }
    println!("finished!");


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
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Collider::cuboid(1000., 10.),
        RigidBody::Fixed,
    )).insert(GlobalTransform::from(Transform::from_translation(Vec3{x: 0., y:-200., z:0.})));


    commands.spawn((RigidBody::Dynamic, Sleeping::disabled()))
        .with_children(|children| {
            children.spawn(Collider::cuboid(10., 10.),)
                // Position the collider relative to the rigid-body.
                .insert(TransformBundle::from(Transform::from_xyz(-100.0, 0.0, 0.0)));
            children.spawn(Collider::cuboid(10., 10.),)
                // Position the collider relative to the rigid-body.
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
            children.spawn(Collider::cuboid(10., 10.),)
                // Position the collider relative to the rigid-body.
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
            children.spawn(Collider::triangle(Vec2::from([130., 0.,]), Vec2::from([130., 130.,]), Vec2::from([0., 130.,])),)
                // Position the collider relative to the rigid-body.
                .insert(TransformBundle::from(Transform::from_xyz(-100.0, -100.0, 0.0)));
        });
}



fn update(

){

}

