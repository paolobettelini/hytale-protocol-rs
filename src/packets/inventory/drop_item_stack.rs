use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct DropItemStack {
    pub slot: i32,
    pub count: i32,
}

impl Packet for DropItemStack {
    const PACKET_ID: u32 = 234;
}

impl PacketRead for DropItemStack {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            slot: buf.read_int_le()?,
            count: buf.read_int_le()?,
        })
    }
}

impl PacketWrite for DropItemStack {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.slot);
        buf.put_i32_le(self.count);
    }
}
