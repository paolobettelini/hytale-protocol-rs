use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct AssetPart {
    pub part: Option<Vec<u8>>,
}

impl Packet for AssetPart {
    const PACKET_ID: u32 = 25;
    fn is_compressed() -> bool {
        true
    }
}

impl PacketRead for AssetPart {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_byte()?;
        let part = if (null_bits & 1) != 0 {
            let len = buf.read_var_int()? as usize;
            if len > 4_096_000 {
                return Err(crate::codec::CodecError::Other("Part too long".to_string()));
            }
            Some(buf.read_bytes(len)?.to_vec())
        } else {
            None
        };

        Ok(AssetPart { part })
    }
}

impl PacketWrite for AssetPart {
    fn write(&self, buf: &mut BytesMut) {
        let null_bits = if self.part.is_some() { 1 } else { 0 };
        buf.put_u8(null_bits);

        if let Some(part) = &self.part {
            crate::codec::write_varint(buf, part.len() as i32);
            buf.put_slice(part);
        }
    }
}
