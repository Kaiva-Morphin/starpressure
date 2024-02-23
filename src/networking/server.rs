use std::{net::{SocketAddr, UdpSocket}, time::SystemTime};

use bevy::ecs::{event::EventReader, system::{Commands, Query, ResMut}};
use bevy_renet::renet::{transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, RenetServer, ServerEvent};
use renet_visualizer::RenetServerVisualizer;

use crate::{channels::{connection_config, Channels}, objects::Object, packets::{ClientDataPacket, ClientGaranteedDataPacket, ServerDataPacket, ServerGaranteedDataPacket}};


pub fn init_server(
    mut commands: Commands,
){
    let server = RenetServer::new(connection_config());
    commands.insert_resource(server);
    let server_addr = vec!["127.0.0.1:6123".parse::<SocketAddr>().unwrap()];
    
    commands.insert_resource(RenetServerVisualizer::<200>::default());
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let socket = UdpSocket::bind(server_addr[0]).unwrap();
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
}



pub fn server_events(
    mut server_events: EventReader<ServerEvent>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    mut server: ResMut<RenetServer>,
){
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                visualizer.add_client(*client_id);
                println!("New client with id {} connected", client_id);
                let encoded: Vec<u8> = bincode::serialize(&ServerGaranteedDataPacket::Connected).unwrap();
                server.send_message(*client_id, Channels::Garanteed, encoded);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                visualizer.remove_client(*client_id);
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}


pub fn update_server(
    mut server: ResMut<RenetServer>,
    q: Query<&Object>
){
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, Channels::Garanteed) {
            let msg: ClientGaranteedDataPacket = bincode::deserialize::<ClientGaranteedDataPacket>(&message).unwrap();
            match msg {
                ClientGaranteedDataPacket::Message { text } => {
                    println!("recieved {} from {}", text, client_id.raw());
                }
                //_ => {}
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, Channels::Fast) {
            let msg: ClientDataPacket = bincode::deserialize::<ClientDataPacket>(&message).unwrap();
            match msg {
                ClientDataPacket::Inputs { keys: _keys } => {

                }
                //_ => {}
            }
        }
    }

    for item in q.iter(){
        println!("{}", item.id());
    }

    for client_id in server.clients_id().into_iter() {
        let msg = ServerDataPacket::Update {
            data: Vec::new(), tick: 0, 
        };
        let encoded: Vec<u8> = bincode::serialize(&msg).unwrap();
        server.send_message(client_id, Channels::Fast, encoded);
    }
}