use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct PlaySoundEvent3D {
    pub sound_event_id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub volume: f32,
    pub pitch: f32,
}

impl Packet for PlaySoundEvent3D {
    const PACKET_ID: u32 = 152;
}

impl PacketRead for PlaySoundEvent3D {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            sound_event_id: buf.read_int_le()?,
            x: buf.read_f32()?,
            y: buf.read_f32()?,
            z: buf.read_f32()?,
            volume: buf.read_f32()?,
            pitch: buf.read_f32()?,
        })
    }
}

impl PacketWrite for PlaySoundEvent3D {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.sound_event_id);
        buf.put_f32_le(self.x);
        buf.put_f32_le(self.y);
        buf.put_f32_le(self.z);
        buf.put_f32_le(self.volume);
        buf.put_f32_le(self.pitch);
    }
}
