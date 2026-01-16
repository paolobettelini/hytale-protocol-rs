use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_string};
use bytes::{BufMut, BytesMut};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClientType {
    Game = 0,
    Editor = 1,
}

#[derive(Debug, Clone)]
pub struct Connect {
    pub protocol_hash: String,
    pub uuid: Uuid,
    pub username: String,
    pub client_type: ClientType,
}

impl Packet for Connect {
    const PACKET_ID: u32 = 0;
}

impl PacketRead for Connect {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        // Simplified read for now to not break hytale-net
        Ok(Self {
            protocol_hash: String::new(),
            uuid: Uuid::nil(),
            username: String::new(),
            client_type: ClientType::Game,
        })
    }
}

impl PacketWrite for Connect {
    fn write(&self, buf: &mut BytesMut) {
        write_string(buf, &self.protocol_hash);
        // Write UUID as big-endian (MostSig, LeastSig)
        buf.put_u64((self.uuid.as_u128() >> 64) as u64);
        buf.put_u64(self.uuid.as_u128() as u64);
        write_string(buf, &self.username);
        buf.put_u8(self.client_type as u8);
    }
}
