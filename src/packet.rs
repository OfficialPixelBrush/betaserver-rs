use std::net::TcpStream;

use crate::network;

pub enum Packet {
    KeepAlive,
    Login {
        entity_id: i32,
        unused: String,
        world_seed: i64,
        dimension: i8,
    },
    Handshake {
        hash: String,
    },
    ChatMessage {
        message: String,
    },
    PlayerPositionLook {
        x: f64,
        y: f64,
        cam_y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool
    },
}

impl Packet {
    pub fn id(&self) -> u8 {
        match self {
            Packet::KeepAlive => 0x00,
            Packet::Login { .. } => 0x01,
            Packet::Handshake { .. } => 0x02,
            Packet::ChatMessage { .. } => 0x03,
            Packet::PlayerPositionLook { .. } => 0x0D,
        }
    }

    pub fn serialize(&self, stream: &mut TcpStream) {
        network::send_u8(stream, self.id());
        match self {
            Packet::KeepAlive => {}

            Packet::Login {
                entity_id,
                unused,
                world_seed,
                dimension,
            } => {
                network::send_i32(stream, *entity_id);
                network::send_str16(stream, unused);
                network::send_i64(stream, *world_seed);
                network::send_i8(stream, *dimension);
            }

            Packet::Handshake { hash } => {
                network::send_str16(stream, hash);
            }

            Packet::ChatMessage { message } => {
                network::send_str16(stream, message);
            }

            Packet::PlayerPositionLook { x,y,cam_y, z, yaw,pitch,on_ground} => {
                network::send_f64(stream, *x);
                network::send_f64(stream, *y);
                network::send_f64(stream, *cam_y);
                network::send_f64(stream, *z);
                network::send_f32(stream, *pitch);
                network::send_f32(stream, *yaw);
                network::send_u8(stream, *on_ground as u8);
            }
        }
    }
}
