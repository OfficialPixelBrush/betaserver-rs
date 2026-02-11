pub trait Packet {
    fn id(&self) -> u8;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Self
    where
        Self: Sized;
}

pub struct KeepAlive;

impl Packet for KeepAlive {
    fn id(&self) -> u8 {
        return 0x00;
    }
    fn serialize(&self) -> Vec<u8> {
        vec![self.id()]
    }
    fn deserialize(_: &[u8]) -> Self {
        Self{}
    }
}

struct Login {
    pub entity_id: i32,
    pub unused: String,
    pub world_seed: i64,
    pub dimension: i8
}

impl Packet for Login {
    fn id(&self) -> u8 {
        return 0x01;
    }
    fn serialize(&self) -> Vec<u8> {
        vec![self.id()]
    }
    fn deserialize(_: &[u8]) -> Self {
        Self{
            entity_id: 0,
            unused: "".to_string(),
            world_seed: 0,
            dimension: 0
        }
    }
}

struct Handshake {
    pub hash : String
}

impl Packet for Handshake {
    fn id(&self) -> u8 {
        return 0x02;
    }
    fn serialize(&self) -> Vec<u8> {
        vec![self.id()]
    }
    fn deserialize(_: &[u8]) -> Self {
        Self{hash: "-".to_string()}
    }
}

struct ChatMessage {
    pub message : String
}

impl Packet for ChatMessage {
    fn id(&self) -> u8 {
        return 0x03;
    }
    fn serialize(&self) -> Vec<u8> {
        vec![self.id()]
    }
    fn deserialize(_: &[u8]) -> Self {
        Self{message: "".to_string()}
    }
}