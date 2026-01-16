use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_varint};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct SetChunk {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub local_light: Option<Vec<u8>>,
    pub global_light: Option<Vec<u8>>,
    pub data: Option<Vec<u8>>,
}

impl Packet for SetChunk {
    const PACKET_ID: u32 = 131;

    fn is_compressed() -> bool {
        true
    }
}

impl PacketRead for SetChunk {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        let x = buf.read_int_le()?;
        let y = buf.read_int_le()?;
        let z = buf.read_int_le()?;

        let local_light = if null_bits & 1 != 0 {
            let len = buf.read_varint()? as usize;
            let bytes = buf.read_bytes(len)?;
            Some(bytes.to_vec())
        } else {
            None
        };

        let global_light = if null_bits & 2 != 0 {
            let len = buf.read_varint()? as usize;
            let bytes = buf.read_bytes(len)?;
            Some(bytes.to_vec())
        } else {
            None
        };

        let data = if null_bits & 4 != 0 {
            let len = buf.read_varint()? as usize;
            let bytes = buf.read_bytes(len)?;
            Some(bytes.to_vec())
        } else {
            None
        };

        Ok(Self {
            x,
            y,
            z,
            local_light,
            global_light,
            data,
        })
    }
}

impl PacketWrite for SetChunk {
    fn write(&self, buf: &mut BytesMut) {
        let mut null_bits = 0u8;
        if self.local_light.is_some() {
            null_bits |= 1;
        }
        if self.global_light.is_some() {
            null_bits |= 2;
        }
        if self.data.is_some() {
            null_bits |= 4;
        }

        buf.put_u8(null_bits);
        buf.put_i32_le(self.x);
        buf.put_i32_le(self.y);
        buf.put_i32_le(self.z);

        if let Some(ref light) = self.local_light {
            write_varint(buf, light.len() as i32);
            buf.put_slice(light);
        }

        if let Some(ref light) = self.global_light {
            write_varint(buf, light.len() as i32);
            buf.put_slice(light);
        }

        if let Some(ref data) = self.data {
            write_varint(buf, data.len() as i32);
            buf.put_slice(data);
        }
    }
}
