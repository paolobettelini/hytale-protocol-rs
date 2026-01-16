use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

/// Ping packet (ID 2)
#[derive(Debug, Clone)]
pub struct Ping {
    pub time: i64,
}

impl Packet for Ping {
    const PACKET_ID: u32 = 2;
}

impl PacketRead for Ping {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            time: buf.read_i64()?,
        })
    }
}

impl PacketWrite for Ping {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i64_le(self.time);
    }
}
