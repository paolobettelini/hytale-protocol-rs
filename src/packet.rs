use crate::codec::{PacketRead, PacketWrite};
use bytes::BytesMut;

pub type PacketId = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    Serverbound,
    Clientbound,
}

pub trait Packet: PacketRead + PacketWrite + Send + Sync + 'static {
    const ID: PacketId;
    const DIRECTION: PacketDirection;
}

pub struct PacketRegistry {}

impl PacketRegistry {
    pub fn new() -> Self {
        Self {}
    }

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
