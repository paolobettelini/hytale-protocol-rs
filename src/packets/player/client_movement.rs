use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct ClientMovement {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl Packet for ClientMovement {
    const PACKET_ID: u32 = 25;
}

impl PacketRead for ClientMovement {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            position_x: buf.read_f64()?,
            position_y: buf.read_f64()?,
            position_z: buf.read_f64()?,
            velocity_x: buf.read_f32()?,
            velocity_y: buf.read_f32()?,
            velocity_z: buf.read_f32()?,
            yaw: buf.read_f32()?,
            pitch: buf.read_f32()?,
            on_ground: buf.read_bool()?,
        })
    }
}

impl PacketWrite for ClientMovement {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_f64_le(self.position_x);
        buf.put_f64_le(self.position_y);
        buf.put_f64_le(self.position_z);
        buf.put_f32_le(self.velocity_x);
        buf.put_f32_le(self.velocity_y);
        buf.put_f32_le(self.velocity_z);
        buf.put_f32_le(self.yaw);
        buf.put_f32_le(self.pitch);
        buf.put_u8(if self.on_ground { 1 } else { 0 });
    }
}
