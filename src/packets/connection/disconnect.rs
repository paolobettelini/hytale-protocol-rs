use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DisconnectType {
    Leave = 0,
    Kick = 1,
    ServerShutdown = 2,
}

#[derive(Debug, Clone)]
pub struct Disconnect {
    pub disconnect_type: DisconnectType,
}

impl Packet for Disconnect {
    const PACKET_ID: u32 = 1;
}

impl PacketRead for Disconnect {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let disconnect_type = match buf.read_u8()? {
            1 => DisconnectType::Kick,
            2 => DisconnectType::ServerShutdown,
            _ => DisconnectType::Leave,
        };
        Ok(Self { disconnect_type })
    }
}

impl PacketWrite for Disconnect {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_u8(self.disconnect_type as u8);
    }
}
