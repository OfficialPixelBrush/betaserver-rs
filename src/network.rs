use std::io::{self,Read};
use std::net::TcpStream;
use std::io::Write;
use crate::Packet;

fn receive_u8(stream: &mut TcpStream) -> u8 {
    let mut buf : [u8; 1] = [0];
    let _ = stream.read_exact(&mut buf);
    return buf[0]
}

fn receive_i8(stream: &mut TcpStream) -> i8 {
    let mut buf : [u8; 1] = [0];
    let _ = stream.read_exact(&mut buf);
    return buf[0] as i8;
}

fn receive_i16(stream: &mut TcpStream) -> i16 {
    let mut buf : [u8; 2] = [0,0];
    let _ = stream.read_exact(&mut buf);
    return i16::from_be_bytes(buf);
}

fn receive_i32(stream: &mut TcpStream) -> i32 {
    let mut buf : [u8; 4] = [0,0,0,0];
    let _ = stream.read_exact(&mut buf);
    return i32::from_be_bytes(buf);
}

fn receive_i64(stream: &mut TcpStream) -> i64 {
    let mut buf : [u8; 8] = [0,0,0,0,0,0,0,0];
    let _ = stream.read_exact(&mut buf);
    return i64::from_be_bytes(buf);
}

fn receive_f32(stream: &mut TcpStream) -> f32 {
    let mut buf : [u8; 4] = [0,0,0,0];
    let _ = stream.read_exact(&mut buf);
    return f32::from_be_bytes(buf);
}

fn receive_f64(stream: &mut TcpStream) -> f64 {
    let mut buf : [u8; 8] = [0,0,0,0,0,0,0,0];
    let _ = stream.read_exact(&mut buf);
    return f64::from_be_bytes(buf);
}

fn receive_str16(stream: &mut TcpStream) -> String {
    let length = receive_i16(stream) as usize;
    let mut buf = vec![0u8; length * 2];
    let _ = stream.read_exact(&mut buf);
    let utf16: Vec<u16> = buf
        .chunks_exact(2)
        .map(|b| u16::from_be_bytes([b[0], b[1]]))
        .collect();
    return String::from_utf16(&utf16).unwrap();
}

pub fn send_u8(stream: &mut TcpStream, val: u8) {
    stream.write_all(&[val]).unwrap();
}

pub fn send_i8(stream: &mut TcpStream, val: i8) {
    stream.write_all(&[val as u8]).unwrap();
}

pub fn send_i16(stream: &mut TcpStream, val: i16) {
    let buf: [u8;2] = i16::to_be_bytes(val);
    stream.write_all(&buf).unwrap();
}

pub fn send_i32(stream: &mut TcpStream, val: i32) {
    let buf: [u8;4] = i32::to_be_bytes(val);
    stream.write_all(&buf).unwrap();
}

pub fn send_i64(stream: &mut TcpStream, val: i64) {
    let buf: [u8;8] = i64::to_be_bytes(val);
    stream.write_all(&buf).unwrap();
}

pub fn send_f32(stream: &mut TcpStream, val: f32) {
    let buf: [u8;4] = f32::to_be_bytes(val);
    stream.write_all(&buf).unwrap();
}

pub fn send_f64(stream: &mut TcpStream, val: f64) {
    let buf: [u8;8] = f64::to_be_bytes(val);
    stream.write_all(&buf).unwrap();
}

pub fn send_str16(stream: &mut TcpStream, s: &str) {
    // Convert Rust string to UTF-16
    let utf16: Vec<u16> = s.encode_utf16().collect();

    // Send length as i16
    let length = utf16.len() as i16;
    stream.write_all(&length.to_be_bytes()).unwrap();

    // Send each u16 as 2 bytes (big-endian)
    for code_unit in utf16 {
        stream.write_all(&code_unit.to_be_bytes()).unwrap();
    }
}

pub fn receive(stream: &mut TcpStream) -> io::Result<Packet> {
    let packet_id = receive_u8(stream);

    let packet = match packet_id {
        1 => Packet::Login { entity_id: receive_i32(stream), unused: receive_str16(stream), world_seed: receive_i64(stream), dimension: receive_i8(stream) },
        2 => Packet::Handshake { hash: receive_str16(stream) },
        3 => Packet::ChatMessage { message: receive_str16(stream) },
        _ => Packet::KeepAlive,
    };
    return Ok(packet);
}

pub fn send(packet: &Packet, stream: &mut TcpStream) {
    let _ = &packet.serialize(stream);
}