//! Common types used in packets.

use crate::codec::{CodecResult, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

/// Represents an asset with a hash and a name.
#[derive(Debug, Clone, PartialEq)]
pub struct Asset {
    pub hash: String,
    pub name: String,
}

impl PacketRead for Asset {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        // hash is fixed 64 bytes ASCII
        let hash_bytes = buf.read_bytes(64)?;
        let hash =
            String::from_utf8(hash_bytes.to_vec()).map_err(crate::codec::CodecError::Utf8)?;

        // name is var string
        let name = buf.read_var_string()?;

        Ok(Asset {
            hash: hash.trim_matches(char::from(0)).to_string(), // Trim null padding if any
            name,
        })
    }
}

impl PacketWrite for Asset {
    fn write(&self, buf: &mut BytesMut) {
        // Write hash (fixed 64 bytes)
        let mut hash_bytes = [0u8; 64];
        let bytes = self.hash.as_bytes();
        let len = bytes.len().min(64);
        hash_bytes[..len].copy_from_slice(&bytes[..len]);
        buf.put_slice(&hash_bytes);

        // Write name
        crate::codec::write_string(buf, &self.name);
    }
}
