use std::{net::UdpSocket, time::SystemTime};

use bevy::{prelude::*, tasks::futures_lite::future::yield_now};

use bevy_egui::EguiContexts;
use bevy_rapier2d::{dynamics::{RigidBody, Velocity}, geometry::Collider};
use bevy_renet::renet::{transport::{ClientAuthentication, NetcodeClientTransport}, RenetClient};
use renet_visualizer::RenetClientVisualizer;
use crate::{channels::{connection_config, Channels}, game::spawn_plyer_puppet, game_core::game_core::GameManager, packets::{ClientDataPacket, ClientGaranteedDataPacket, ServerDataPacket, ServerGaranteedDataPacket}, InputKeys};
use bevy::input::ButtonInput;




pub fn init_client(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_manager: ResMut<GameManager>,
){
    let server_addr = "127.0.0.1:9100".parse().unwrap();
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
    commands.spawn(Camera2dBundle::default());
    let poe = spawn_plyer_puppet(&mut commands, &mut meshes, &mut materials, Color::RED);
    commands.entity(poe).insert((
        game_manager.new_object(poe),
        ServerSide
    ));
    let poe = spawn_plyer_puppet(&mut commands, &mut meshes, &mut materials, Color::BLUE);
    commands.entity(poe).insert((
        game_manager.new_object(poe),
        Transform::from_translation(Vec3::Z),
        Player
    ));
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct ServerSide;

pub fn update_client(
    mut client: ResMut<RenetClient>,
    mut transport: ResMut<NetcodeClientTransport>,
    mut serverside_q: Query<(&mut Velocity, &mut Transform), (With<ServerSide>, Without<Player>)>,
    mut player_q: Query<(&mut Velocity, &mut Transform), (With<Player>, Without<ServerSide>)>,
    keys: Res<ButtonInput<KeyCode>>,
    restime: Res<Time>
){
    while let Some(message) = client.receive_message(Channels::Garanteed) {
        let msg: ServerGaranteedDataPacket = bincode::deserialize::<ServerGaranteedDataPacket>(&message).unwrap(); // todo: add exception handle! (for wrong packets! Garanteed in Fast)
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
        let msg: ServerDataPacket = bincode::deserialize::<ServerDataPacket>(&message).unwrap(); // todo: add exception handle! (for wrong packets! Garanteed in Fast)
        match msg{
            ServerDataPacket::Update{data: data, tick: _tick} => { // todo: tick steps on client!
                //println!("{}", _tick);
                let d = data.get(0);
                if let Some(d) = d {
                    let (mut v, mut t) = serverside_q.single_mut();
                    v.linvel = d.linvel;
                    t.translation = d.position;
                }
            },
            //ServerDataPacket::Echo{time} => {/*println!("Delay: {}", restime.elapsed_seconds() - time)*/},
            _ => {}
        }
    }
    let keys = InputKeys{
        up: keys.pressed(KeyCode::KeyW),
        down: keys.pressed(KeyCode::KeyS),
        left: keys.pressed(KeyCode::KeyA),
        right: keys.pressed(KeyCode::KeyD)
    };
    let encoded: Vec<u8> = bincode::serialize(&ClientDataPacket::Inputs { keys: keys.clone() }).unwrap();
    client.send_message(Channels::Fast, encoded);
    let mut vec = Vec2::ZERO;
    if keys.up {vec.y += 1.};
    if keys.down {vec.y -= 1.};
    if keys.left {vec.x -= 1.};
    if keys.right {vec.x += 1.};
    let p = player_q.get_single_mut();
    if let Ok(t) = p{
        let (mut v, t_) = t;
        v.linvel += vec * 3.; 
        if vec == Vec2::ZERO {v.linvel *= 0.9};
    }
}

pub fn send_message(
    mut client: ResMut<RenetClient>,
    restime: Res<Time>,
    mut game_manager: ResMut<GameManager>,
){
    /*let encoded: Vec<u8> = bincode::serialize(&ClientDataPacket::Echo { time: restime.elapsed_seconds() }).unwrap();
    client.send_message(Channels::Fast, encoded);*/
    
    let encoded: Vec<u8> = bincode::serialize(&ClientGaranteedDataPacket::Message { text: format!("now: {}", game_manager.get_tick()) }).unwrap();
    client.send_message(Channels::Garanteed, encoded);
}

pub fn update_client_visualizer(
    mut visualizer: ResMut<RenetClientVisualizer<200>>,
    mut egui_contexts: EguiContexts,
    mut client: ResMut<RenetClient>,
){
    visualizer.add_network_info(client.network_info());
    visualizer.show_window(egui_contexts.ctx_mut());
}