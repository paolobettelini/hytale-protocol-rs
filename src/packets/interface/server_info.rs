use crate::codec::{Packet, PacketRead, PacketWrite, PacketBuffer, CodecResult, write_string};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub server_name: String,
    pub motd: Option<String>,
    pub max_players: i32,
}

impl Packet for ServerInfo {
    const PACKET_ID: u32 = 223;
}

impl PacketRead for ServerInfo {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        let server_name = buf.read_var_string()?;
        let max_players = buf.read_int_le()?;
        
        let motd = if null_bits & 1 != 0 {
            Some(buf.read_var_string()?)
        } else {
            None
        };
        
        Ok(Self {
            server_name,
            motd,
            max_players,
        })
    }
}

impl PacketWrite for ServerInfo {
    fn write(&self, buf: &mut BytesMut) {
        let null_bits = if self.motd.is_some() { 1u8 } else { 0u8 };
        buf.put_u8(null_bits);
        
        write_string(buf, &self.server_name);
        buf.put_i32_le(self.max_players);
        
        if let Some(ref motd) = self.motd {
            write_string(buf, motd);
        }
    }
}
