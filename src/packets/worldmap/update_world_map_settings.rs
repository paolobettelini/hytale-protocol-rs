use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct UpdateWorldMapSettings {
    pub enabled: bool,
    pub allow_coords: bool,
    pub allow_markers: bool,
    pub default_scale: f32,
    pub min_scale: f32,
    pub max_scale: f32,
}

impl Packet for UpdateWorldMapSettings {
    const PACKET_ID: u32 = 240;
}

impl PacketRead for UpdateWorldMapSettings {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let _null_bits = buf.read_u8()?; // biomeDataMap not supported yet
        Ok(Self {
            enabled: buf.read_bool()?,
            allow_coords: buf.read_bool()?,
            allow_markers: buf.read_bool()?,
            default_scale: buf.read_f32()?,
            min_scale: buf.read_f32()?,
            max_scale: buf.read_f32()?,
        })
    }
}

impl PacketWrite for UpdateWorldMapSettings {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_u8(0); // nullBits (no biomeDataMap)
        buf.put_u8(if self.enabled { 1 } else { 0 });
        buf.put_u8(if self.allow_coords { 1 } else { 0 });
        buf.put_u8(if self.allow_markers { 1 } else { 0 });
        buf.put_f32_le(self.default_scale);
        buf.put_f32_le(self.min_scale);
        buf.put_f32_le(self.max_scale);
    }
}
