use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct ViewRadius {
    pub value: i32,
}

impl Packet for ViewRadius {
    const PACKET_ID: u32 = 32;
}

impl PacketRead for ViewRadius {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            value: buf.read_int_le()?,
        })
    }
}

impl PacketWrite for ViewRadius {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.value);
    }
}
