use bevy::prelude::*;

mod networking;
mod game_core;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use std::env;

use networking::{networking::*, *};
use game_core::*;




const SERVER_TPS: f64 = 1.; 

fn main(){
    let mut app = App::new();
    let args: Vec<String> = env::args().collect();
    app.insert_resource(Time::<Fixed>::from_seconds(2.));
    if args.contains(&String::from("server")){
        app.add_plugins(Server);
    } else {
        app.add_plugins(Client);
    }
    app.run();
}

