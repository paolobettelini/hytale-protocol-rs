use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct SetActiveSlot {
    pub slot: i32,
}

impl Packet for SetActiveSlot {
    const PACKET_ID: u32 = 233;
}

impl PacketRead for SetActiveSlot {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            slot: buf.read_int_le()?,
        })
    }
}

impl PacketWrite for SetActiveSlot {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.slot);
    }
}
