use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::BytesMut;

/// ClientReady packet (ID 24)
/// Sent by client to signal readiness to receive world data.
#[derive(Debug, Clone)]
pub struct ClientReady;

impl Packet for ClientReady {
    const PACKET_ID: u32 = 24;
}

impl PacketRead for ClientReady {
    fn read(_buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self)
    }
}

impl PacketWrite for ClientReady {
    fn write(&self, _buf: &mut BytesMut) {}
}
