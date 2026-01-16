use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::BytesMut;

/// WorldLoadFinished packet (ID 22)
#[derive(Debug, Clone)]
pub struct WorldLoadFinished {}

impl Packet for WorldLoadFinished {
    const PACKET_ID: u32 = 22;
}

impl PacketRead for WorldLoadFinished {
    fn read(_buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(WorldLoadFinished {})
    }
}

impl PacketWrite for WorldLoadFinished {
    fn write(&self, _buf: &mut BytesMut) {
    }
}
