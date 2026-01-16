use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PongType {
    Pong = 0,
}

/// Pong packet (ID 3)
#[derive(Debug, Clone)]
pub struct Pong {
    pub pong_type: PongType,
    pub time: i64,
}

impl Packet for Pong {
    const PACKET_ID: u32 = 3;
}

impl PacketRead for Pong {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let pong_type = match buf.read_u8()? {
            _ => PongType::Pong,
        };
        Ok(Self {
            pong_type,
            time: buf.read_i64()?,
        })
    }
}

impl PacketWrite for Pong {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_u8(self.pong_type as u8);
        buf.put_i64_le(self.time);
    }
}
