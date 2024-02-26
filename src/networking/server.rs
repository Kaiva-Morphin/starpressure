use std::{net::{SocketAddr, UdpSocket}, time::SystemTime};

use bevy::{asset::Assets, core_pipeline::core_2d::Camera2dBundle, ecs::{entity::Entity, event::EventReader, query::With, system::{Commands, Local, Query, Res, ResMut}}, math::Vec2, render::{color::Color, mesh::Mesh}, sprite::ColorMaterial, transform::components::Transform, utils::HashMap};
use bevy_egui::EguiContexts;
use bevy_rapier2d::dynamics::Velocity;
use bevy_renet::renet::{transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, RenetServer, ServerEvent};
use renet_visualizer::RenetServerVisualizer;

use crate::{channels::{connection_config, Channels}, game::spawn_plyer_puppet, game_core::game_core::GameManager, objects::Object, packets::{ClientDataPacket, ClientGaranteedDataPacket, ServerDataPacket, ServerGaranteedDataPacket}, ObjectData};


pub fn init_server(
    mut commands: Commands,
){

    let server = RenetServer::new(connection_config());
    commands.insert_resource(server);
    let server_addr = vec!["37.110.11.176:9100".parse::<SocketAddr>().unwrap(), "192.168.0.100:9100".parse::<SocketAddr>().unwrap(), "127.0.0.1:9100".parse::<SocketAddr>().unwrap(), ];
    
    commands.insert_resource(RenetServerVisualizer::<200>::default());
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let socket = UdpSocket::bind(server_addr[2]).unwrap();
    const GAME_PROTOCOL_ID: u64 = 0;
    let server_config = ServerConfig {
        max_clients: 16,
        protocol_id: GAME_PROTOCOL_ID,
        public_addresses: server_addr,
        current_time: current_time,
        authentication: ServerAuthentication::Unsecure // todo: change to secure
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    commands.insert_resource(transport);
    println!("server started");
    commands.spawn(Camera2dBundle::default());
}



pub fn server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_manager: ResMut<GameManager>,
    mut players: Local<HashMap<u64, Entity>>, // client_id -> entity
){
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                visualizer.add_client(*client_id);
                println!("New client with id {} connected", client_id);
                let encoded: Vec<u8> = bincode::serialize(&ServerGaranteedDataPacket::Connected).unwrap();
                server.send_message(*client_id, Channels::Garanteed, encoded);

                let poe = spawn_plyer_puppet(&mut commands, &mut meshes, &mut materials, Color::RED);
                commands.entity(poe).insert(
                    game_manager.new_object(poe)
                );

                players.insert(client_id.raw(), poe);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                visualizer.remove_client(*client_id);
                println!("Client {client_id} disconnected: {reason}");
                let poe = players.get(&client_id.raw());
                if let Some(poe) = poe {
                    if let Some(mut c) = commands.get_entity(*poe){
                        c.despawn();
                    }
                }
            }
        }
    }
}

pub fn update_visualizer(
    server: Res<RenetServer>,
    mut egui_contexts: EguiContexts,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
){
    visualizer.update(&server);
    visualizer.show_window(egui_contexts.ctx_mut());
}

pub fn update_server(
    mut server: ResMut<RenetServer>,
    mut game_manager: ResMut<GameManager>,
    mut q: Query<(&mut Velocity, &Transform)>
){
    game_manager.tick_step();
    for client_id in server.clients_id().into_iter() { 
        // todo: add exceptions for packet spam! for example, (velocity += inputs.velocity),
        // todo: but if many packets recieved it may looks like (velocity += inputs.velocity * 5)
        while let Some(message) = server.receive_message(client_id, Channels::Garanteed) {
            let msg: ClientGaranteedDataPacket = bincode::deserialize::<ClientGaranteedDataPacket>(&message).unwrap(); // todo: add exception handle!
            match msg {
                ClientGaranteedDataPacket::Message { text } => {
                    println!("recieved {} from {}", text, client_id.raw());
                }
                //_ => {}
            }
        }
    }

    for client_id in server.clients_id().into_iter() { 
        // todo: add exceptions for packet spam! for example, (velocity += inputs.velocity),
        // todo: but if many packets recieved it may looks like (velocity += inputs.velocity * 5)
        while let Some(message) = server.receive_message(client_id, Channels::Fast) {
            
            let msg: ClientDataPacket = bincode::deserialize::<ClientDataPacket>(&message).unwrap(); // todo: add exception handle!
            match msg {
                ClientDataPacket::Inputs { keys } => {
                    let t = q.get_single_mut();
                    if let Ok(t) = t{
                        let (mut v, t) = t;
                        let mut vec = Vec2::ZERO;
                        if keys.up {vec.y += 1.};
                        if keys.down {vec.y -= 1.};
                        if keys.left {vec.x -= 1.};
                        if keys.right {vec.x += 1.};
                        v.linvel += vec * 3.;
                        if vec == Vec2::ZERO {v.linvel *= 0.9};
                    }
                }
                /*ClientDataPacket::Echo{time} => {
                    let msg = ServerDataPacket::Echo { time };
                    let encoded: Vec<u8> = bincode::serialize(&msg).unwrap();
                    server.send_message(client_id, Channels::Fast, encoded);
                }*/
                _ => {}
            }
        }
    }

    /*for item in q.iter(){
        println!("{}", item.id());
    }*/

    for client_id in server.clients_id().into_iter() {
        let t = q.get_single_mut();
        if let Ok(t) = t{
            let (mut v, t) = t;
            let msg = ServerDataPacket::Update {
                data: vec![ObjectData{linvel: v.linvel, position: t.translation}], tick: 0, 
            };
            let encoded: Vec<u8> = bincode::serialize(&msg).unwrap();
            server.send_message(client_id, Channels::Fast, encoded);
        }
    }
}