use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::BytesMut;

#[derive(Debug, Clone)]
pub struct RequestCommonAssetsRebuild {}

impl Packet for RequestCommonAssetsRebuild {
    const PACKET_ID: u32 = 38;
}

impl PacketRead for RequestCommonAssetsRebuild {
    fn read(_buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(RequestCommonAssetsRebuild {})
    }
}

impl PacketWrite for RequestCommonAssetsRebuild {
    fn write(&self, _buf: &mut BytesMut) {}
}
