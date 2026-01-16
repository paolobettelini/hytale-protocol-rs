use crate::codec::write_string;
use bytes::{BufMut, BytesMut};

pub struct EntityEffect {
    pub id: Option<String>,
    pub name: Option<String>,
    pub world_removal_sound_event_index: i32,
    pub local_removal_sound_event_index: i32,
    pub duration: f32,
    pub infinite: bool,
    pub debuff: bool,
    pub overlap_behavior: u8, // Enum: Extend(0), Overwrite(1), Ignore(2)
    pub damage_calculator_cooldown: f64,
    pub value_type: u8, // Enum: Percent(0), Absolute(1)
                        // Optional fields defaulted to None
                        // pub application_effects: Option<ApplicationEffects>,
                        // pub model_override: Option<ModelOverride>,
                        // pub status_effect_icon: Option<String>,
                        // pub stat_modifiers: Option<HashMap<i32, f32>>,
}

impl Default for EntityEffect {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            world_removal_sound_event_index: -1,
            local_removal_sound_event_index: -1,
            duration: 0.0,
            infinite: false,
            debuff: false,
            overlap_behavior: 0, // Extend
            damage_calculator_cooldown: 0.0,
            value_type: 0, // Percent
        }
    }
}

impl EntityEffect {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: Some(id.to_string()),
            name: Some(name.to_string()),
            ..Default::default()
        }
    }

    pub fn write(&self, buf: &mut BytesMut) {
        // Layout:
        // 0: NullBits (1 byte)
        // 1: Fixed Block (variable start 49)
        // Fixed Size = 25.
        // Offsets Table (6 entries) = 24.
        // 1 + 25 + 24 = 50. Wait.
        // Java:
        // `VARIABLE_BLOCK_START = 49`.
        // `FIXED_BLOCK_SIZE = 25`.
        // Nullbits (1) + Fixed (25) = 26?
        // Offsets table starts after fixed block.
        // 6 entries * 4 bytes = 24 bytes.
        // 1 + 25 = 26. 26 + 24 = 50.
        // Wait, Java says `offset + 25` is start of table?
        // `int fieldOffset0 = buf.getIntLE(offset + 25);`
        // So Nullbits(0) + Fixed(1..25) = 25 bytes?
        // No, `worldRemovalSoundEventIndex` is at offset+1.
        // `valueType` is at offset+24.
        // So Fixed Block is 1..25 (inclusive range, length 24 bytes).
        // Wait. `ValueType.fromValue(buf.getByte(offset + 24))`.
        // So last byte is at 24.
        // Length 24 bytes (1..24).
        // `FIXED_BLOCK_SIZE` constant says 25. Maybe includes nullbits?
        // Offset table starts at 25.
        // `buf.getIntLE(offset + 25)`.
        // So yes, offsets 0 (Null), 1-24 (Fixed), 25.. (Offsets).
        // 6 offsets * 4 = 24 bytes.
        // 25 + 24 = 49.
        // Matches `VARIABLE_BLOCK_START = 49`.

        let start_pos = buf.len();

        // 1. NullBits (1 byte)
        let null_bits_idx = start_pos;
        buf.put_u8(0);

        // 2. Fixed Data (Offsets 1 to 24)
        buf.put_i32_le(self.world_removal_sound_event_index);
        buf.put_i32_le(self.local_removal_sound_event_index);
        buf.put_f32_le(self.duration);
        buf.put_u8(if self.infinite { 1 } else { 0 });
        buf.put_u8(if self.debuff { 1 } else { 0 });
        buf.put_u8(self.overlap_behavior);
        buf.put_f64_le(self.damage_calculator_cooldown);
        buf.put_u8(self.value_type);

        // 3. Offsets Table (6 entries * 4 bytes)
        let offsets_start_idx = buf.len(); // Should be start_pos + 25
        buf.resize(offsets_start_idx + 24, 0xFF);

        let var_start_idx = buf.len(); // Should be start_pos + 49

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

        // Name (index 1) - nullBits & 2
        if let Some(name) = &self.name {
            null_bits |= 2;
            let current_pos = buf.len();
            let rel_off = (current_pos - var_start_idx) as i32;
            let off_idx = offsets_start_idx + 4;
            let mut t = BytesMut::with_capacity(4);
            t.put_i32_le(rel_off);
            buf[off_idx..off_idx + 4].copy_from_slice(&t);
            write_string(buf, name);
        }

        // ApplicationEffects (index 2) - nullBits & 4

        // ModelOverride (index 3) - nullBits & 8

        // StatusEffectIcon (index 4) - nullBits & 16

        // StatModifiers (index 5) - nullBits & 32

        // Write NullBits
        buf[null_bits_idx] = null_bits;
    }
}
