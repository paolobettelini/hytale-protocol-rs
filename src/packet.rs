//! Packet trait and registry.

use crate::codec::{CodecResult, PacketBuffer, PacketRead, PacketWrite};
use bytes::BytesMut;

/// Packet ID type.
pub type PacketId = u16;

/// Direction of packet flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    /// Client to server.
    Serverbound,
    /// Server to client.
    Clientbound,
}

/// Trait for all packets.
pub trait Packet: PacketRead + PacketWrite + Send + Sync + 'static {
    /// The unique packet ID.
    const ID: PacketId;

    /// The direction this packet flows.
    const DIRECTION: PacketDirection;
}

/// Registry for packet serialization/deserialization.
pub struct PacketRegistry {
    // In a full implementation, this would hold codec functions
    // For now, we use static dispatch via the Packet trait
}

impl PacketRegistry {
    pub fn new() -> Self {
        Self {}
    }

    /// Encode a packet to bytes.
    pub fn encode<P: Packet>(&self, packet: &P) -> BytesMut {
        let mut buf = BytesMut::new();
        // Write packet ID as VarInt
        crate::codec::write_varint(&mut buf, P::ID as i32);
        // Write packet data
        packet.write(&mut buf);
        buf
    }
}

impl Default for PacketRegistry {
    fn default() -> Self {
        Self::new()
    }
}
