use std::{net::UdpSocket, time::SystemTime};

use bevy::{prelude::*, tasks::futures_lite::future::yield_now};
use bevy_renet::renet::{transport::{ClientAuthentication, NetcodeClientTransport}, RenetClient};
use crate::{channels::{connection_config, Channels}, packets::{ClientDataPacket, ClientGaranteedDataPacket, ServerDataPacket, ServerGaranteedDataPacket}};





pub fn init_client(
    mut commands: Commands,
){
    let server_addr = "127.0.0.1:6123".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    const GAME_PROTOCOL_ID: u64 = 0;

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let transport = NetcodeClientTransport::new(
        current_time, 
        ClientAuthentication::Unsecure {
            protocol_id: GAME_PROTOCOL_ID,
            client_id: current_time.as_millis() as u64,
            server_addr: server_addr,
            user_data: None
        }, 
        socket
    ).unwrap();
    commands.insert_resource(RenetClient::new(connection_config()));
    commands.insert_resource(transport);
    println!("client created!");
}



pub fn update_client(
    mut client: ResMut<RenetClient>,
    mut transport: ResMut<NetcodeClientTransport>
){
    //println!("connecting? {}", client.is_connecting());
    //println!("disconnected? {}", client.is_disconnected());
    while let Some(message) = client.receive_message(Channels::Garanteed) {
        let msg: ServerGaranteedDataPacket = bincode::deserialize::<ServerGaranteedDataPacket>(&message).unwrap();
        match msg{
            ServerGaranteedDataPacket::Connected{} => {
                let message = String::from("Halo!");
                let encoded: Vec<u8> = bincode::serialize(&ClientGaranteedDataPacket::Message { text: message }).unwrap();
                client.send_message(Channels::Garanteed, encoded)
            }
            ServerGaranteedDataPacket::Message { text } => {println!("recieved text: {}", text)}
        }
    }

    while let Some(message) = client.receive_message(Channels::Fast) {
        let msg: ServerDataPacket = bincode::deserialize::<ServerDataPacket>(&message).unwrap();
        match msg{
            ServerDataPacket::Update{data: _data, tick: _tick} => {},
            ServerDataPacket::Echo{time} => {println!("Delay: {}", time - SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f32())}
        }
    }
}

pub fn send_message(
    mut client: ResMut<RenetClient>,
){
    let encoded: Vec<u8> = bincode::serialize(&ClientDataPacket::Echo { time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f32() }).unwrap();
    client.send_message(Channels::Fast, encoded);
}

