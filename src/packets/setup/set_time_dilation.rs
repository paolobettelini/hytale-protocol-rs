use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

/// SetTimeDilation packet (ID 30)
#[derive(Debug, Clone)]
pub struct SetTimeDilation {
    pub time_dilation: f32,
}

impl Packet for SetTimeDilation {
    const PACKET_ID: u32 = 30;
}

impl PacketRead for SetTimeDilation {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            time_dilation: buf.read_f32()?,
        })
    }
}

impl PacketWrite for SetTimeDilation {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_f32_le(self.time_dilation);
    }
}
