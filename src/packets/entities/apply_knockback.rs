use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct ApplyKnockback {
    pub entity_id: i32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
}

impl Packet for ApplyKnockback {
    const PACKET_ID: u32 = 88;
}

impl PacketRead for ApplyKnockback {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            entity_id: buf.read_int_le()?,
            velocity_x: buf.read_f32()?,
            velocity_y: buf.read_f32()?,
            velocity_z: buf.read_f32()?,
        })
    }
}

impl PacketWrite for ApplyKnockback {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.entity_id);
        buf.put_f32_le(self.velocity_x);
        buf.put_f32_le(self.velocity_y);
        buf.put_f32_le(self.velocity_z);
    }
}
