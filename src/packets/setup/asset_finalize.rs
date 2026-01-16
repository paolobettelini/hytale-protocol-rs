use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::BytesMut;

#[derive(Debug, Clone, Default)]
pub struct AssetFinalize;

impl Packet for AssetFinalize {
    const PACKET_ID: u32 = 26;
}

impl PacketRead for AssetFinalize {
    fn read(_buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(AssetFinalize)
    }
}

impl PacketWrite for AssetFinalize {
    fn write(&self, _buf: &mut BytesMut) {
        // Empty body
    }
}
