struct KeepAlive {
    id: u8 = 0x00
}

impl KeepAlive {
    fn serialize(&self) -> [u8; 1] {
        return [self.id];
    }
}

struct Login {
    id: u8 = 0x01,
    entity_id: i32,
    unused: String,
    world_seed: i64,
    dimension: i8
}

struct Handshake {
    id: u8 = 0x02,
    hash : String
}

struct ChatMessage {
    id: u8 = 0x03,
    message : String
}