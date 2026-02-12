mod packet;
mod network;
mod primitives;
mod player;

use std::net::{TcpListener, TcpStream};

use crate::packet::Packet;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_connection(&mut stream);
    }
}

fn handle_connection(stream: &mut TcpStream) {
    loop {
        let packet = match network::receive(stream) {
            Ok(p) => p,
            Err(_) => {
                println!("Connection closed or error");
                break;
            }
        };
        match packet {
            Packet::KeepAlive => {
                println!("Keep Alive");
            }

            Packet::Login { entity_id, .. } => {
                println!("Login: {}", entity_id);
                network::send(&Packet::Login { entity_id: 0, unused: "".to_string(), world_seed: 0, dimension: 0 }, stream);
                network::send(&Packet::PlayerPositionLook { x:0.0, y: 64.0, cam_y: 64.0+1.65, z: 0.0, yaw: 0.0, pitch: 0.0, on_ground: false }, stream);
            }

            Packet::Handshake { hash } => {
                println!("Handshake: {}", hash);
                network::send(&Packet::Handshake { hash: "-".to_string() }, stream)
            }

            Packet::ChatMessage { message } => {
                println!("Chat: {}", message);
                network::send(&Packet::ChatMessage { message },stream);
            }

            Packet::PlayerPosition { x, y, cam_y, z, on_ground } => {
                //println!("Pos: ({}, {}, {})", x,y,z);
            }

            Packet::PlayerLook { yaw, pitch, on_ground } => {
                //println!("Look: ({}, {})", yaw, pitch);
            }

            Packet::PlayerPositionLook { x, y, cam_y, z, yaw, pitch, on_ground } => {
                //println!("PosLook: ({}, {}, {}) ({}, {})", x,y,z, yaw, pitch);
            }

            Packet::Disconnect { reason } => {
                println!("Disconnected: {}", reason);
                break;
            }

            _ => println!("Unimplemented: #{}", packet.id()),
        }
    }
}