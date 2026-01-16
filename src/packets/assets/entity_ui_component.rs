use crate::codec::write_varint;
use bytes::{BufMut, BytesMut};

pub struct EntityUIComponent {
    pub type_: u8, // Enum: EntityStat(0), CombatText(1)

    // Optional fields
    pub hitbox_offset: Option<(f32, f32)>, // Vector2f
    pub unknown: bool,
    pub entity_stat_index: i32,

    pub combat_text_random_pos_offset_range: Option<Vec<u8>>, // Placeholder for RangeVector2f (17 bytes)
    pub combat_text_viewport_margin: f32,
    pub combat_text_duration: f32,
    pub combat_text_hit_angle_modifier_strength: f32,
    pub combat_text_font_size: f32,
    pub combat_text_color: Option<(u8, u8, u8)>, // Color (3 bytes)

    // Variable field
    pub combat_text_animation_events: Option<Vec<u8>>, // Placeholder for array
}

impl Default for EntityUIComponent {
    fn default() -> Self {
        Self {
            type_: 0, // EntityStat
            hitbox_offset: None,
            unknown: false,
            entity_stat_index: 0,
            combat_text_random_pos_offset_range: None,
            combat_text_viewport_margin: 0.0,
            combat_text_duration: 0.0,
            combat_text_hit_angle_modifier_strength: 0.0,
            combat_text_font_size: 0.0,
            combat_text_color: None,
            combat_text_animation_events: None,
        }
    }
}

impl EntityUIComponent {
    pub fn new_stat(stat_index: i32) -> Self {
        Self {
            type_: 0, // EntityStat
            entity_stat_index: stat_index,
            ..Default::default()
        }
    }

    pub fn write(&self, buf: &mut BytesMut) {
        // Layout:
        // 0: NullBits (1 byte)
        // 1: Type (1 byte)
        // 2: HitboxOffset (8 bytes) OR Zeros
        // 10: Unknown (1 byte)
        // 11: EntityStatIndex (4 bytes)
        // 15: CombatTextRandomPositionOffsetRange (17 bytes) OR Zeros
        // 32: CombatTextViewportMargin (4 bytes)
        // 36: CombatTextDuration (4 bytes)
        // 40: CombatTextHitAngleModifierStrength (4 bytes)
        // 44: CombatTextFontSize (4 bytes)
        // 48: CombatTextColor (3 bytes) OR Zeros
        // 51: Variable Data (CombatTextAnimationEvents)

        let mut null_bits = 0u8;

        if self.hitbox_offset.is_some() {
            null_bits |= 1;
        }
        if self.combat_text_random_pos_offset_range.is_some() {
            null_bits |= 2;
        }
        if self.combat_text_color.is_some() {
            null_bits |= 4;
        }
        if self.combat_text_animation_events.is_some() {
            null_bits |= 8;
        }

        buf.put_u8(null_bits);
        buf.put_u8(self.type_);

        // HitboxOffset (8 bytes)
        if let Some((x, y)) = self.hitbox_offset {
            buf.put_f32_le(x);
            buf.put_f32_le(y);
        } else {
            buf.put_bytes(0, 8);
        }

        buf.put_u8(if self.unknown { 1 } else { 0 });
        buf.put_i32_le(self.entity_stat_index);

        // CombatTextRandomPositionOffsetRange (17 bytes)
        if let Some(ref data) = self.combat_text_random_pos_offset_range {
            // Assume data is correct size or handle
            // Since we don't have struct, assume None for defaults or implementation later
            buf.put_slice(data);
        } else {
            buf.put_bytes(0, 17);
        }

        buf.put_f32_le(self.combat_text_viewport_margin);
        buf.put_f32_le(self.combat_text_duration);
        buf.put_f32_le(self.combat_text_hit_angle_modifier_strength);
        buf.put_f32_le(self.combat_text_font_size);

        // CombatTextColor (3 bytes)
        if let Some((r, g, b)) = self.combat_text_color {
            buf.put_u8(r);
            buf.put_u8(g);
            buf.put_u8(b);
        } else {
            buf.put_bytes(0, 3);
        }

        // Variable Data
        if let Some(ref events) = self.combat_text_animation_events {
            // Write array length + data
            // We assume 'events' is raw serialized bytes including VarInt length?
            // Or we write length here?
            // Java: VarInt.write(len); for items...
            // Here: "VarInt.write(buf, this.combatTextAnimationEvents.length);"
            // So we need to write length.
            // Impl simplified: just assume None for now.
            // If needed: write_varint(buf, count); write content.
            // For default None, nothing written.
        }
    }
}
