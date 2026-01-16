use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite};
use crate::common::Asset;
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct AssetInitialize {
    pub asset: Asset,
    pub size: i32,
}

impl Packet for AssetInitialize {
    const PACKET_ID: u32 = 24;
}

impl PacketRead for AssetInitialize {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let size = buf.read_int_le()?;
        let asset = Asset::read(buf)?;

        Ok(AssetInitialize { asset, size })
    }
}

impl PacketWrite for AssetInitialize {
    fn write(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.size);
        self.asset.write(buf);
    }
}
