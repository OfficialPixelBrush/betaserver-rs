use core::net;
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
    Time {
        time: i64,
    },
    EntityEquipment {
        entity_id: i32,
        inventory_slot: i16,
        item_id: i16,
        item_meta: i16,
    },
    SpawnPoint {
        x: i32,
        y: i32,
        z: i32,
    },
    ClickEntity {
        player_id: i32,
        entity_id: i32,
        left_click: bool,
    },
    SetHealth {
        health: i16
    },
    Respawn {
        dimension: i8
    },
    PlayerOnGround {
        on_ground: bool,
    },
    PlayerPosition {
        x: f64,
        y: f64,
        cam_y: f64,
        z: f64,
        on_ground: bool,
    },
    PlayerLook {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    PlayerPositionLook {
        x: f64,
        y: f64,
        cam_y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    Mine {
        status : i8,
        x : i32,
        y : i8,
        z : i32,
        face : i8,
    },
    Place {
        x : i32,
        y : i8,
        z : i32,
        face : i8,
        item_id : i16,
        item_amount : i8,
        item_meta : i16,
    },
    Disconnect {
        reason : String
    }
}

impl Packet {
    pub fn id(&self) -> u8 {
        match self {
            Packet::KeepAlive => 0x00,
            Packet::Login { .. } => 0x01,
            Packet::Handshake { .. } => 0x02,
            Packet::ChatMessage { .. } => 0x03,
            Packet::Time { .. } => 0x04,
            Packet::EntityEquipment { .. } => 0x05,
            Packet::SpawnPoint { .. } => 0x06,
            Packet::ClickEntity { .. } => 0x07,
            Packet::SetHealth { .. } => 0x08,
            Packet::Respawn { .. } => 0x09,
            Packet::PlayerOnGround { .. } => 0x0A,
            Packet::PlayerPosition { .. } => 0x0B,
            Packet::PlayerLook { .. } => 0x0C,
            Packet::PlayerPositionLook { .. } => 0x0D,
            Packet::Mine { .. } => 0x0E,
            Packet::Place { .. } => 0x0F,
            Packet::Disconnect { .. } => 0xFF,
        }
    }

    pub fn serialize(&self, stream: &mut TcpStream) {
        network::send_u8(stream, self.id());
        match self {
            Packet::KeepAlive => {}
            Packet::Login { entity_id, unused, world_seed, dimension } => {
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
            Packet::Time { time } => {
                network::send_i64(stream, *time);
            },
            Packet::EntityEquipment { entity_id, inventory_slot, item_id, item_meta } => todo!(),
            Packet::SpawnPoint { x, y, z } => todo!(),
            Packet::ClickEntity { player_id, entity_id, left_click } => todo!(),
            Packet::SetHealth { health } => {
                network::send_i16(stream, *health);
            },
            Packet::Respawn { dimension } => {
                network::send_i8(stream, *dimension);
            },
            Packet::PlayerOnGround { on_ground } => todo!(),
            Packet::PlayerPosition { x, y, cam_y, z, on_ground } => todo!(),
            Packet::PlayerLook { yaw, pitch, on_ground } => todo!(),
            Packet::PlayerPositionLook { x, y, cam_y, z, yaw,pitch,on_ground} => {
                network::send_f64(stream, *x);
                network::send_f64(stream, *y);
                network::send_f64(stream, *cam_y);
                network::send_f64(stream, *z);
                network::send_f32(stream, *pitch);
                network::send_f32(stream, *yaw);
                network::send_bool(stream, *on_ground);
            }
            Packet::Mine { status, x, y, z, face } => {
                network::send_i8(stream, *status);
                network::send_i32(stream, *x);
                network::send_i8(stream, *y);
                network::send_i32(stream, *z);
                network::send_i8(stream, *face);
            },
            Packet::Place { x, y, z, face, item_id, item_amount, item_meta } => {
                network::send_i32(stream, *x);
                network::send_i8(stream, *y);
                network::send_i32(stream, *z);
                network::send_i8(stream, *face);
                network::send_i16(stream, *item_id);
                if (*item_id > (-1 as i16)) {
                    network::send_i8(stream, *item_amount);
                    network::send_i16(stream, *item_meta);
                }
            },
            Packet::Disconnect { reason } => {
                network::send_str16(stream, reason);
            }
        }
    }
}
