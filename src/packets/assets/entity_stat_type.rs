use crate::codec::write_string;
use bytes::{BufMut, BytesMut};

pub struct EntityStatType {
    pub id: Option<String>,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub reset_behavior: u8, // Enum: InitialValue(0), MaxValue(1)
                            // Optional fields (we will default to None for now)
                            // pub min_value_effects: Option<EntityStatEffects>,
                            // pub max_value_effects: Option<EntityStatEffects>,
}

impl Default for EntityStatType {
    fn default() -> Self {
        Self {
            id: None,
            value: 100.0,
            min: 0.0,
            max: 100.0,
            reset_behavior: 0, // InitialValue
        }
    }
}

impl EntityStatType {
    pub fn new(id: &str) -> Self {
        Self {
            id: Some(id.to_string()),
            ..Default::default()
        }
    }

    pub fn write(&self, buf: &mut BytesMut) {
        // Layout:
        // 0: NullBits (1 byte)
        // 1: Fixed Block (bytes 1..26? No, 14 bytes fixed. variable start 26.)
        // Java:
        // byte nullBits = buf.getByte(offset);
        // obj.value = buf.getFloatLE(offset + 1);
        // obj.min = buf.getFloatLE(offset + 5);
        // obj.max = buf.getFloatLE(offset + 9);
        // obj.resetBehavior = ... (offset + 13)
        // Fixed Block Size = 14.
        // Offsets Table starts at offset 14.
        // 3 entries * 4 bytes = 12 bytes.
        // 14 + 12 = 26.
        // Variable data starts at 26.

        let start_pos = buf.len();

        // 1. NullBits
        let null_bits_idx = start_pos;
        buf.put_u8(0);

        // 2. Fixed Data and Offsets Table
        // We need space for 1 + 13 (Fixed) + 12 (Offsets) = 26 bytes total header.
        // Wait, buf.put_u8(0) wrote 1 byte.
        // We need 25 more bytes.
        let fixed_start = start_pos; // including nullbits

        // Value (4 bytes)
        buf.put_f32_le(self.value);
        // Min (4 bytes)
        buf.put_f32_le(self.min);
        // Max (4 bytes)
        buf.put_f32_le(self.max);
        // ResetBehavior (1 byte)
        buf.put_u8(self.reset_behavior);

        // Offsets Table (3 entries * 4 bytes = 12 bytes)
        // Initialize to -1
        let offsets_start_idx = buf.len();
        buf.resize(offsets_start_idx + 12, 0xFF);

        let var_start_idx = buf.len(); // Should be start_pos + 26
        let mut null_bits = 0u8;

        // ID (index 0) - nullBits & 1
        if let Some(id) = &self.id {
            null_bits |= 1;
            let current_pos = buf.len();
            let rel_off = (current_pos - var_start_idx) as i32;
            let off_idx = offsets_start_idx + 0;
            let mut t = BytesMut::with_capacity(4);
            t.put_i32_le(rel_off);
            buf[off_idx..off_idx + 4].copy_from_slice(&t);
            write_string(buf, id);
        }

        // MinValueEffects (index 1) - nullBits & 2
        // Default None/Skipped

        // MaxValueEffects (index 2) - nullBits & 4
        // Default None/Skipped

        // Write NullBits
        buf[null_bits_idx] = null_bits;
    }
}
