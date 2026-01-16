use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_string};
use bytes::{BufMut, BytesMut};

/// WorldLoadProgress packet (ID 21)
#[derive(Debug, Clone)]
pub struct WorldLoadProgress {
    pub percent_complete: i32,
    pub percent_complete_subitem: i32,
    pub status: Option<String>,
}

impl Packet for WorldLoadProgress {
    const PACKET_ID: u32 = 21;
}

impl PacketRead for WorldLoadProgress {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        let percent_complete = buf.read_int_le()?;
        let percent_complete_subitem = buf.read_int_le()?;

        let status = if (null_bits & 1) != 0 {
            Some(buf.read_var_string()?)
        } else {
            None
        };

        Ok(WorldLoadProgress {
            percent_complete,
            percent_complete_subitem,
            status,
        })
    }
}

impl PacketWrite for WorldLoadProgress {
    fn write(&self, buf: &mut BytesMut) {
        let mut null_bits = 0u8;
        if self.status.is_some() {
            null_bits |= 1;
        }

        buf.put_u8(null_bits);
        buf.put_i32_le(self.percent_complete);
        buf.put_i32_le(self.percent_complete_subitem);

        if let Some(status) = &self.status {
            write_string(buf, status);
        }
    }
}
