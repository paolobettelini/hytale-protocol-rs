use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

/// ServerSetBlock packet (ID 150)
#[derive(Debug, Clone)]
pub struct ServerSetBlock {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block_state_id: i32,
}

impl Packet for ServerSetBlock {
    const PACKET_ID: u32 = 150;
}

impl PacketRead for ServerSetBlock {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            x: buf.read_int_le()?,
            y: buf.read_int_le()?,
            z: buf.read_int_le()?,
            block_state_id: buf.read_int_le()?,
        })
    }
}

impl PacketWrite for ServerSetBlock {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.x);
        buf.put_i32_le(self.y);
        buf.put_i32_le(self.z);
        buf.put_i32_le(self.block_state_id);
    }
}
