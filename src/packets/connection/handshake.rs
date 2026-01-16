//! Handshake packets.

use crate::codec::{
    CodecResult, PacketBuffer, PacketRead, PacketWrite, write_string, write_varint,
};
use crate::packet::{Packet, PacketDirection, PacketId};
use bytes::BufMut;

/// Initial handshake from client.
#[derive(Debug, Clone)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeNextState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeNextState {
    Status = 1,
    Login = 2,
}

impl PacketRead for HandshakePacket {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let protocol_version = buf.read_varint()?;
        let server_address = buf.read_string()?;
        let server_port = buf.read_u16()?;
        let next_state = match buf.read_varint()? {
            1 => HandshakeNextState::Status,
            2 => HandshakeNextState::Login,
            n => return Err(crate::codec::CodecError::InvalidEnumVariant(n)),
        };

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}

impl PacketWrite for HandshakePacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        write_varint(buf, self.protocol_version);
        write_string(buf, &self.server_address);
        buf.put_u16(self.server_port);
        write_varint(buf, self.next_state as i32);
    }
}

impl Packet for HandshakePacket {
    const ID: PacketId = 0x00;
    const DIRECTION: PacketDirection = PacketDirection::Serverbound;
}
