use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_varint};
use bytes::{BufMut, BytesMut};

/// UpdateFeatures packet (ID 31)
#[derive(Debug, Clone)]
pub struct UpdateFeatures {
    pub features: Option<Vec<(u8, bool)>>,
}

impl Packet for UpdateFeatures {
    const PACKET_ID: u32 = 31;
}

impl PacketRead for UpdateFeatures {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        if (null_bits & 1) != 0 {
            let count = buf.read_varint()? as usize;
            let mut features = Vec::with_capacity(count);
            for _ in 0..count {
                let feature = buf.read_u8()?;
                let enabled = buf.read_bool()?;
                features.push((feature, enabled));
            }
            Ok(Self { features: Some(features) })
        } else {
            Ok(Self { features: None })
        }
    }
}

impl PacketWrite for UpdateFeatures {
    fn write(&self, buf: &mut BytesMut) {
        if let Some(ref features) = self.features {
            buf.put_u8(1);
            write_varint(buf, features.len() as i32);
            for (feature, enabled) in features {
                buf.put_u8(*feature);
                buf.put_u8(if *enabled { 1 } else { 0 });
            }
        } else {
            buf.put_u8(0);
        }
    }
}
