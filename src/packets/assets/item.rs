use crate::codec::write_string;
use bytes::{BufMut, BytesMut};

pub struct Item {
    pub id: Option<String>,
    pub block_id: i32,
    pub icon: Option<String>,
}

impl Item {
    pub fn new(id: &str) -> Self {
        Self {
            id: Some(id.to_string()),
            block_id: -1,
            icon: None,
        }
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn with_block_id(mut self, block_id: i32) -> Self {
        self.block_id = block_id;
        self
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(512);

        // NullBits (4 bytes) - mark ALL mandatory fields
        let mut null_bits: [u8; 4] = [0, 0, 0, 0];
        if self.id.is_some() {
            null_bits[0] |= 1;
        }
        null_bits[0] |= 16; // playerAnimationsId ALWAYS
        if self.icon.is_some() {
            null_bits[0] |= 32;
        }
        null_bits[2] |= 1; // itemEntity ALWAYS
        null_bits[2] |= 128; // interactions ALWAYS
        null_bits[3] |= 1; // interactionVars ALWAYS
        null_bits[3] |= 2; // interactionConfig ALWAYS

        buf.put_slice(&null_bits);

        // Fixed Block
        buf.put_f32_le(1.0);
        buf.put_u8(0);
        buf.put_i32_le(64);
        buf.put_i32_le(0); // reticleIndex = 0
        buf.put_slice(&[0u8; 25]);
        buf.put_i32_le(0);
        buf.put_i32_le(0); // qualityIndex = 0
        buf.put_u8(0);
        buf.put_u8(0);
        buf.put_i32_le(self.block_id);
        buf.put_slice(&[0u8; 16]);
        buf.put_slice(&[0u8; 4]);
        buf.put_slice(&[0u8; 4]);
        buf.put_f64_le(0.0);
        buf.put_i32_le(0);
        buf.put_i32_le(0); // soundEventIndex, itemSoundSetIndex = 0
        buf.put_slice(&[0u8; 49]);
        buf.put_u8(0);
        buf.put_u8(0);

        // Offset table
        let id_slot = buf.len();
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        let player_id_slot = buf.len();
        buf.put_i32_le(-1);
        let icon_slot = buf.len();
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        let entity_slot = buf.len();
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        let interact_slot = buf.len();
        buf.put_i32_le(-1);
        let interact_vars_slot = buf.len();
        buf.put_i32_le(-1);
        let interact_cfg_slot = buf.len();
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);

        let start = buf.len();

        if let Some(ref id) = self.id {
            let off = (buf.len() - start) as i32;
            buf[id_slot..id_slot + 4].copy_from_slice(&off.to_le_bytes());
            write_string(&mut buf, id);
        }

        let off = (buf.len() - start) as i32;
        buf[player_id_slot..player_id_slot + 4].copy_from_slice(&off.to_le_bytes());
        write_string(&mut buf, "default");

        if let Some(ref icon) = self.icon {
            let off = (buf.len() - start) as i32;
            buf[icon_slot..icon_slot + 4].copy_from_slice(&off.to_le_bytes());
            write_string(&mut buf, icon);
        }

        let off = (buf.len() - start) as i32;
        buf[entity_slot..entity_slot + 4].copy_from_slice(&off.to_le_bytes());
        buf.put_u8(1);
        buf.put_slice(&[0u8; 3]);
        buf.put_u8(1);
        write_string(&mut buf, "Item");

        let off = (buf.len() - start) as i32;
        buf[interact_slot..interact_slot + 4].copy_from_slice(&off.to_le_bytes());
        buf.put_u8(1);
        buf.put_u8(13);
        buf.put_i32_le(0); // 13 = SwapFrom

        let off = (buf.len() - start) as i32;
        buf[interact_vars_slot..interact_vars_slot + 4].copy_from_slice(&off.to_le_bytes());
        buf.put_u8(0);

        let off = (buf.len() - start) as i32;
        buf[interact_cfg_slot..interact_cfg_slot + 4].copy_from_slice(&off.to_le_bytes());
        buf.put_u8(0);
        buf.put_u8(1);
        buf.put_u8(0);
        buf.put_u8(0);
        buf.put_i32_le(-1);
        buf.put_i32_le(-1);

        buf.to_vec()
    }
}
