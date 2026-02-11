use std::net::TcpStream;
use std::io::Write;
use crate::packet::Packet;
use crate::packet::KeepAlive;

fn receive(mut stream: TcpStream) -> Box<dyn Packet> {
    return Box::new(KeepAlive)
}

fn send<T: Packet>(value: T, mut stream: TcpStream) {
    stream.write_all(&value.serialize()).unwrap();
}