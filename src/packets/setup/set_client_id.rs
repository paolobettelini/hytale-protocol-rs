use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct SetClientId {
    pub client_id: i32,
}

impl Packet for SetClientId {
    const PACKET_ID: u32 = 100;
}

impl PacketRead for SetClientId {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(SetClientId {
            client_id: buf.read_int_le()?,
        })
    }
}

impl PacketWrite for SetClientId {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.client_id);
    }
}
