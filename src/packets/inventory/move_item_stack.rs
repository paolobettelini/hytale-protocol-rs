use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

/// MoveItemStack packet (ID 235)
#[derive(Debug, Clone)]
pub struct MoveItemStack {
    pub from_slot: i32,
    pub to_slot: i32,
    pub count: i32,
}

impl Packet for MoveItemStack {
    const PACKET_ID: u32 = 235;
}

impl PacketRead for MoveItemStack {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            from_slot: buf.read_int_le()?,
            to_slot: buf.read_int_le()?,
            count: buf.read_int_le()?,
        })
    }
}

impl PacketWrite for MoveItemStack {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.from_slot);
        buf.put_i32_le(self.to_slot);
        buf.put_i32_le(self.count);
    }
}
