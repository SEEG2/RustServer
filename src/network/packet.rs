pub trait Packet {
    type InterpretType;
    const PACKET_ID: u16;
    fn create(content: Self::InterpretType) -> Result<Box<[u8]>, &'static str>;
    fn interpret(content: Box<[u8]>) -> Result<Self::InterpretType, &'static str>;
}

pub struct TestPacket;
impl Packet for TestPacket {
    type InterpretType = String;
    const PACKET_ID: u16 = 0;

    fn create(content: Self::InterpretType) -> Result<Box<[u8]>, &'static str> {
        if content.len() > 1 {
            Err("Content size exceeds max packet size")
        } else {
            Ok(create_raw(Self::PACKET_ID, content.as_bytes()))
        }
    }

    fn interpret(content: Box<[u8]>) -> Result<Self::InterpretType, &'static str> {
        if content.len() > 1 {
            Err("Content is too large to be interpreted as this packet")
        } else {
            match String::from_utf8(Vec::from(content)) {
                Ok(result) => Ok(result.to_string()) ,
                Err(_) => Err("Invalid UTF-8"),
            }
        }
    }
}

pub fn create_raw(packet_id: u16, slice: &[u8]) -> Box<[u8]> {
    let mut combined = Vec::with_capacity(2 + slice.len());
    combined.extend_from_slice(&packet_id.to_le_bytes());
    combined.extend_from_slice(slice);
    combined.into_boxed_slice()
}