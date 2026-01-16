use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_string};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct AuthGrant {
    pub authorization_grant: Option<String>,
    pub server_identity_token: Option<String>,
}

impl Packet for AuthGrant {
    const PACKET_ID: u32 = 11;
}

impl PacketRead for AuthGrant {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        // Offsets (2 * 4 bytes)
        buf.read_int_le()?;
        buf.read_int_le()?;

        let authorization_grant = if (null_bits & 1) != 0 {
            Some(buf.read_var_string()?)
        } else {
            None
        };

        let server_identity_token = if (null_bits & 2) != 0 {
            Some(buf.read_var_string()?)
        } else {
            None
        };

        Ok(AuthGrant {
            authorization_grant,
            server_identity_token,
        })
    }
}

impl PacketWrite for AuthGrant {
    fn write(&self, buf: &mut BytesMut) {
        let mut null_bits: u8 = 0;
        if self.authorization_grant.is_some() {
            null_bits |= 1;
        }
        if self.server_identity_token.is_some() {
            null_bits |= 2;
        }
        buf.put_u8(null_bits);

        let mut field_data = BytesMut::new();
        let mut offsets = [0i32; 2];

        if let Some(ref s) = self.authorization_grant {
            offsets[0] = field_data.len() as i32;
            write_string(&mut field_data, s);
        } else {
            offsets[0] = -1;
        }

        if let Some(ref s) = self.server_identity_token {
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
