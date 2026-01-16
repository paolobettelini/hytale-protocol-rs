use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

/// SetEntitySeed packet (ID 160)
#[derive(Debug, Clone)]
pub struct SetEntitySeed {
    pub entity_seed: i32,
}

impl Packet for SetEntitySeed {
    const PACKET_ID: u32 = 160;
}

impl PacketRead for SetEntitySeed {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(SetEntitySeed {
            entity_seed: buf.read_int_le()?,
        })
    }
}

impl PacketWrite for SetEntitySeed {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.entity_seed);
    }
}
