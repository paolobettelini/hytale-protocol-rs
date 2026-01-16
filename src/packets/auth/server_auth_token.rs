use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_string, write_varint};
use bytes::{BufMut, BytesMut};

/// ServerAuthToken packet (ID 13) - Final auth packet from server.
#[derive(Debug, Clone)]
pub struct ServerAuthToken {
    pub server_access_token: Option<String>,
    pub password_challenge: Option<Vec<u8>>,
}

impl Packet for ServerAuthToken {
    const PACKET_ID: u32 = 13;
}

impl PacketRead for ServerAuthToken {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        // Offsets
        buf.read_int_le()?;
        buf.read_int_le()?;

        let server_access_token = if (null_bits & 1) != 0 {
            Some(buf.read_var_string()?)
        } else {
            None
        };

        let password_challenge = if (null_bits & 2) != 0 {
            let len = buf.read_varint()? as usize;
            Some(buf.read_bytes(len)?.to_vec())
        } else {
            None
        };

        Ok(ServerAuthToken {
            server_access_token,
            password_challenge,
        })
    }
}

impl PacketWrite for ServerAuthToken {
    fn write(&self, buf: &mut BytesMut) {
        let mut null_bits: u8 = 0;
        if self.server_access_token.is_some() { null_bits |= 1; }
        if self.password_challenge.is_some() { null_bits |= 2; }
        buf.put_u8(null_bits);

        let mut field_data = BytesMut::new();
        let mut offsets = [0i32; 2];

        if let Some(ref s) = self.server_access_token {
            offsets[0] = field_data.len() as i32;
            write_string(&mut field_data, s);
        } else {
            offsets[0] = -1;
        }

        if let Some(ref challenge) = self.password_challenge {
            offsets[1] = field_data.len() as i32;
            write_varint(&mut field_data, challenge.len() as i32);
            field_data.put_slice(challenge);
        } else {
            offsets[1] = -1;
        }

        for offset in offsets {
            buf.put_i32_le(offset);
        }
        buf.put_slice(&field_data);
    }
}
