use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JoinWorld {
    pub clear_world: bool,
    pub fade_in_out: bool,
    pub world_uuid: Uuid,
}

impl Packet for JoinWorld {
    const PACKET_ID: u32 = 104;
}

impl PacketRead for JoinWorld {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            clear_world: buf.read_bool()?,
            fade_in_out: buf.read_bool()?,
            world_uuid: Uuid::from_u128(
                ((buf.read_u64()? as u128) << 64) | (buf.read_u64()? as u128),
            ),
        })
    }
}

impl PacketWrite for JoinWorld {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_u8(if self.clear_world { 1 } else { 0 });
        buf.put_u8(if self.fade_in_out { 1 } else { 0 });
        buf.put_u64((self.world_uuid.as_u128() >> 64) as u64);
        buf.put_u64(self.world_uuid.as_u128() as u64);
    }
}
