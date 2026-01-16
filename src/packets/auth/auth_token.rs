use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_string};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct AuthToken {
    pub access_token: Option<String>,
    pub server_authorization_grant: Option<String>,
}

impl Packet for AuthToken {
    const PACKET_ID: u32 = 12;
}

impl PacketRead for AuthToken {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        // Offsets
        buf.read_int_le()?;
        buf.read_int_le()?;

        let access_token = if (null_bits & 1) != 0 {
            Some(buf.read_var_string()?)
        } else {
            None
        };

        let server_authorization_grant = if (null_bits & 2) != 0 {
            Some(buf.read_var_string()?)
        } else {
            None
        };

        Ok(AuthToken {
            access_token,
            server_authorization_grant,
        })
    }
}

impl PacketWrite for AuthToken {
    fn write(&self, buf: &mut BytesMut) {
        let mut null_bits: u8 = 0;
        if self.access_token.is_some() {
            null_bits |= 1;
        }
        if self.server_authorization_grant.is_some() {
            null_bits |= 2;
        }
        buf.put_u8(null_bits);

        let mut field_data = BytesMut::new();
        let mut offsets = [0i32; 2];

        if let Some(ref s) = self.access_token {
            offsets[0] = field_data.len() as i32;
            write_string(&mut field_data, s);
        } else {
            offsets[0] = -1;
        }

        if let Some(ref s) = self.server_authorization_grant {
            offsets[1] = field_data.len() as i32;
            write_string(&mut field_data, s);
        } else {
            offsets[1] = -1;
        }

        for offset in offsets {
            buf.put_i32_le(offset);
        }
        buf.put_slice(&field_data);
    }
}

impl AuthToken {
    /// Helper to deserialize from bytes directly (legacy support)
    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        let mut buf = PacketBuffer::new(bytes::Bytes::copy_from_slice(data));
        Self::read(&mut buf).map_err(|e| e.to_string())
    }
}
