use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_string};
use crate::common::Asset;
use bytes::{BufMut, BytesMut};

/// WorldSettings packet (ID 21)
#[derive(Debug, Clone)]
pub struct WorldSettings {
    pub game_version: String,
    pub world_seed: i64,
    pub time_of_day: f64,
    pub weather_seed: i64,
    pub world_height: i32,
    pub required_assets: Option<Vec<Asset>>,
}

impl Packet for WorldSettings {
    const PACKET_ID: u32 = 21;
    fn is_compressed() -> bool { true }
}

impl PacketRead for WorldSettings {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        let game_version = buf.read_var_string()?;
        let time_of_day = buf.read_f64()?;
        let world_seed = buf.read_i64()?;
        let weather_seed = buf.read_i64()?;
        let world_height = buf.read_int_le()?;

        let required_assets = if (null_bits & 1) != 0 {
            let count = buf.read_varint()? as usize;
            let mut assets = Vec::with_capacity(count);
            for _ in 0..count {
                assets.push(Asset::read(buf)?);
            }
            Some(assets)
        } else {
            None
        };

        Ok(WorldSettings {
            game_version,
            world_seed,
            time_of_day,
            weather_seed,
            world_height,
            required_assets,
        })
    }
}

impl PacketWrite for WorldSettings {
    fn write(&self, buf: &mut BytesMut) {
        let mut null_bits = 0u8;
        if self.required_assets.is_some() { null_bits |= 1; }
        
        buf.put_u8(null_bits);
        write_string(buf, &self.game_version);
        buf.put_f64_le(self.time_of_day);
        buf.put_i64_le(self.world_seed);
        buf.put_i64_le(self.weather_seed);
        buf.put_i32_le(self.world_height);

        if let Some(ref assets) = self.required_assets {
            crate::codec::write_varint(buf, assets.len() as i32);
            for asset in assets {
                asset.write(buf);
            }
        }
    }
}
