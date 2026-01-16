use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

/// ClientTeleport packet (ID 26)
/// Sent by client to confirm a teleportation or update orientation.
#[derive(Debug, Clone)]
pub struct ClientTeleport {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl Packet for ClientTeleport {
    const PACKET_ID: u32 = 26;
}

impl PacketRead for ClientTeleport {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            position_x: buf.read_f64()?,
            position_y: buf.read_f64()?,
            position_z: buf.read_f64()?,
            yaw: buf.read_f32()?,
            pitch: buf.read_f32()?,
        })
    }
}

impl PacketWrite for ClientTeleport {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_f64_le(self.position_x);
        buf.put_f64_le(self.position_y);
        buf.put_f64_le(self.position_z);
        buf.put_f32_le(self.yaw);
        buf.put_f32_le(self.pitch);
    }
}
