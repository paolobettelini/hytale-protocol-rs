use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct SetUpdateRate {
    pub updates_per_second: f32,
}

impl Packet for SetUpdateRate {
    const PACKET_ID: u32 = 29;
}

impl PacketRead for SetUpdateRate {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            updates_per_second: buf.read_f32()?,
        })
    }
}

impl PacketWrite for SetUpdateRate {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_f32_le(self.updates_per_second);
    }
}
