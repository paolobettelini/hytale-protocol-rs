use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

/// PlaySoundEvent2D packet (ID 151)
#[derive(Debug, Clone)]
pub struct PlaySoundEvent2D {
    pub sound_event_id: i32,
    pub volume: f32,
    pub pitch: f32,
}

impl Packet for PlaySoundEvent2D {
    const PACKET_ID: u32 = 151;
}

impl PacketRead for PlaySoundEvent2D {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            sound_event_id: buf.read_int_le()?,
            volume: buf.read_f32()?,
            pitch: buf.read_f32()?,
        })
    }
}

impl PacketWrite for PlaySoundEvent2D {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.sound_event_id);
        buf.put_f32_le(self.volume);
        buf.put_f32_le(self.pitch);
    }
}
