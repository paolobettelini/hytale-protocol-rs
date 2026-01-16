use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct UpdateWorldMap {
    // empty markers/chunks for now
}

impl Packet for UpdateWorldMap {
    const PACKET_ID: u32 = 241;
}

impl PacketRead for UpdateWorldMap {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let _null_bits = buf.read_u8()?;
        buf.read_int_le()?; // chunksOffset
        buf.read_int_le()?; // addedMarkersOffset
        buf.read_int_le()?; // removedMarkersOffset
        Ok(Self {})
    }
}

impl PacketWrite for UpdateWorldMap {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_u8(0); // nullBits = 0
        buf.put_i32_le(-1); // chunksOffset
        buf.put_i32_le(-1); // addedMarkersOffset
        buf.put_i32_le(-1); // removedMarkersOffset
    }
}
