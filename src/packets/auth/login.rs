use crate::codec::{
    CodecResult, PacketBuffer, PacketRead, PacketWrite, write_string, write_varint,
};
use crate::packet::{Packet, PacketDirection, PacketId};
use bytes::BufMut;

#[derive(Debug, Clone)]
pub struct LoginStartPacket {
    pub username: String,
    pub uuid: Option<u128>,
}

impl PacketRead for LoginStartPacket {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let username = buf.read_string()?;
        let uuid = if buf.remaining() >= 16 {
            let hi = buf.read_u64()?;
            let lo = buf.read_u64()?;
            Some(((hi as u128) << 64) | (lo as u128))
        } else {
            None
        };

        Ok(Self { username, uuid })
    }
}

impl PacketWrite for LoginStartPacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        write_string(buf, &self.username);
        if let Some(uuid) = self.uuid {
            buf.put_u64((uuid >> 64) as u64);
            buf.put_u64(uuid as u64);
        }
    }
}

impl Packet for LoginStartPacket {
    const ID: PacketId = 0x00;
    const DIRECTION: PacketDirection = PacketDirection::Serverbound;
}

#[derive(Debug, Clone)]
pub struct LoginSuccessPacket {
    pub uuid: u128,
    pub username: String,
}

impl PacketRead for LoginSuccessPacket {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let hi = buf.read_u64()?;
        let lo = buf.read_u64()?;
        let uuid = ((hi as u128) << 64) | (lo as u128);
        let username = buf.read_string()?;

        Ok(Self { uuid, username })
    }
}

impl PacketWrite for LoginSuccessPacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        buf.put_u64((self.uuid >> 64) as u64);
        buf.put_u64(self.uuid as u64);
        write_string(buf, &self.username);
        // Properties (empty array)
        write_varint(buf, 0);
    }
}

impl Packet for LoginSuccessPacket {
    const ID: PacketId = 0x02;
    const DIRECTION: PacketDirection = PacketDirection::Clientbound;
}

/// Disconnect during login.
#[derive(Debug, Clone)]
pub struct LoginDisconnectPacket {
    pub reason: String,
}

impl PacketRead for LoginDisconnectPacket {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let reason = buf.read_string()?;
        Ok(Self { reason })
    }
}

impl PacketWrite for LoginDisconnectPacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        write_string(buf, &self.reason);
    }
}

impl Packet for LoginDisconnectPacket {
    const ID: PacketId = 0x00;
    const DIRECTION: PacketDirection = PacketDirection::Clientbound;
}
