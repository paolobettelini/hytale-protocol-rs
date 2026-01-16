use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_varint};
use crate::common::Asset;
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct RequestAssets {
    pub assets: Option<Vec<Asset>>,
}

impl Packet for RequestAssets {
    const PACKET_ID: u32 = 17;
    fn is_compressed() -> bool {
        true
    }
}

impl PacketRead for RequestAssets {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        if (null_bits & 1) != 0 {
            let count = buf.read_varint()? as usize;
            let mut assets = Vec::with_capacity(count);
            for _ in 0..count {
                assets.push(Asset::read(buf)?);
            }
            Ok(Self {
                assets: Some(assets),
            })
        } else {
            Ok(Self { assets: None })
        }
    }
}

impl PacketWrite for RequestAssets {
    fn write(&self, buf: &mut BytesMut) {
        if let Some(ref assets) = self.assets {
            buf.put_u8(1);
            write_varint(buf, assets.len() as i32);
            for asset in assets {
                asset.write(buf);
            }
        } else {
            buf.put_u8(0);
        }
    }
}
