use crate::codec::{write_string, write_varint};
use bytes::{BufMut, BytesMut};

pub struct BlockType {
    pub item: Option<String>,
    pub name: String,
    pub unknown: bool,
    pub draw_type: u8, // Enum
    pub material: u8,  // Enum
    pub opacity: u8,   // Enum
    // We only implement fields needed for defaults for now
    pub cube_textures: Option<Vec<BlockTextures>>,
}

pub struct BlockTextures {
    pub top: String,
    pub bottom: String,
    pub front: String,
    pub back: String,
    pub left: String,
    pub right: String,
    pub weight: f32,
}

impl BlockTextures {
    pub fn write(&self, buf: &mut BytesMut) {
        use crate::codec::write_string;
        use bytes::BufMut;

        // BlockTextures Layout:
        // 0: nullBits (1 byte)
        // 1: weight (4 bytes)
        // 5: offsets (6 * 4 = 24 bytes) (variable strings)
        // 29: variable data start

        let start_pos = buf.len();

        buf.put_u8(0);

        buf.put_f32_le(self.weight);

        let offsets_start = buf.len(); // should be start_pos + 5
        buf.resize(offsets_start + 24, 0xFF);

        let var_start = buf.len(); // should be start_pos + 29

        let mut null_bits = 0u8;

        // ID 0: Top
        // ID 1: Bottom
        // ID 2: Front
        // ID 3: Back
        // ID 4: Left
        // ID 5: Right

        // Field IDs correspond to: 1<<0, 1<<1, 1<<2...

        // Top (ID 0)
        if !self.top.is_empty() {
            null_bits |= 1;
            let current_pos = buf.len();
            let rel_off = (current_pos - var_start) as i32;
            let off_idx = offsets_start + (0 * 4);
            let mut t = BytesMut::with_capacity(4);
            t.put_i32_le(rel_off);
            buf[off_idx..off_idx + 4].copy_from_slice(&t);
            write_string(buf, &self.top);
        }

        // Bottom (ID 1)
        if !self.bottom.is_empty() {
            null_bits |= 2;
            let current_pos = buf.len();
            let rel_off = (current_pos - var_start) as i32;
            let off_idx = offsets_start + (1 * 4);
            let mut t = BytesMut::with_capacity(4);
            t.put_i32_le(rel_off);
            buf[off_idx..off_idx + 4].copy_from_slice(&t);
            write_string(buf, &self.bottom);
        }

        // Front (ID 2)
        if !self.front.is_empty() {
            null_bits |= 4;
            let current_pos = buf.len();
            let rel_off = (current_pos - var_start) as i32;
            let off_idx = offsets_start + (2 * 4);
            let mut t = BytesMut::with_capacity(4);
            t.put_i32_le(rel_off);
            buf[off_idx..off_idx + 4].copy_from_slice(&t);
            write_string(buf, &self.front);
        }

        // Back (ID 3)
        if !self.back.is_empty() {
            null_bits |= 8;
            let current_pos = buf.len();
            let rel_off = (current_pos - var_start) as i32;
            let off_idx = offsets_start + (3 * 4);
            let mut t = BytesMut::with_capacity(4);
            t.put_i32_le(rel_off);
            buf[off_idx..off_idx + 4].copy_from_slice(&t);
            write_string(buf, &self.back);
        }

        // Left (ID 4)
        if !self.left.is_empty() {
            null_bits |= 16;
            let current_pos = buf.len();
            let rel_off = (current_pos - var_start) as i32;
            let off_idx = offsets_start + (4 * 4);
            let mut t = BytesMut::with_capacity(4);
            t.put_i32_le(rel_off);
            buf[off_idx..off_idx + 4].copy_from_slice(&t);
            write_string(buf, &self.left);
        }

        // Right (ID 5)
        if !self.right.is_empty() {
            null_bits |= 32;
            let current_pos = buf.len();
            let rel_off = (current_pos - var_start) as i32;
            let off_idx = offsets_start + (5 * 4);
            let mut t = BytesMut::with_capacity(4);
            t.put_i32_le(rel_off);
            buf[off_idx..off_idx + 4].copy_from_slice(&t);
            write_string(buf, &self.right);
        }

        // Write NullBits
        buf[start_pos] = null_bits;
    }
}

impl Default for BlockType {
    fn default() -> Self {
        Self {
            item: None,
            name: "Empty".to_string(),
            unknown: false,
            draw_type: 0, // Empty
            material: 0,  // Empty
            opacity: 3,   // Transparent (was 0/Solid)
            cube_textures: None,
        }
    }
}

impl BlockType {
    pub fn air() -> Self {
        Self::default()
    }

    pub fn unknown() -> Self {
        Self {
            item: Some("unknown".to_string()),
            name: "Unknown".to_string(),
            unknown: true,
            draw_type: 2, // Cube (was 1/GizmoCube)
            material: 1,  // Solid
            opacity: 0,   // Solid
            cube_textures: Some(vec![BlockTextures {
                top: "BlockTextures/Unknown.png".to_string(),
                bottom: "BlockTextures/Unknown.png".to_string(),
                front: "BlockTextures/Unknown.png".to_string(),
                back: "BlockTextures/Unknown.png".to_string(),
                left: "BlockTextures/Unknown.png".to_string(),
                right: "BlockTextures/Unknown.png".to_string(),
                weight: 1.0,
            }]),
        }
    }

    pub fn grass() -> Self {
        Self {
            item: Some("calcite".to_string()),
            name: "Calcite".to_string(),
            unknown: false,
            draw_type: 2, // Cube
            material: 1,  // Solid
            opacity: 0,   // Solid
            cube_textures: Some(vec![BlockTextures {
                top: "BlockTextures/Calcite_Top.png".to_string(),
                bottom: "BlockTextures/Calcite.png".to_string(),
                front: "BlockTextures/Calcite.png".to_string(),
                back: "BlockTextures/Calcite.png".to_string(),
                left: "BlockTextures/Calcite.png".to_string(),
                right: "BlockTextures/Calcite.png".to_string(),
                weight: 1.0,
            }]),
        }
    }

    pub fn simple(name: &str, item: &str, tex: &str) -> Self {
        Self {
            item: Some(item.to_string()),
            name: name.to_string(),
            unknown: false,
            draw_type: 2, // Cube
            material: 1,  // Solid
            opacity: 0,   // Solid
            cube_textures: Some(vec![BlockTextures {
                top: tex.to_string(),
                bottom: tex.to_string(),
                front: tex.to_string(),
                back: tex.to_string(),
                left: tex.to_string(),
                right: tex.to_string(),
                weight: 1.0,
            }]),
        }
    }

    pub fn write(&self, buf: &mut BytesMut) {
        let start_pos = buf.len();

        // NullBits (4 bytes) - Placeholder, we fill at end
        let null_bits_idx = start_pos;
        buf.put_u32_le(0);

        // 2. Fixed Block (bytes 4..163) (Length 159)
        // We initialize with 0xFF (-1) to ensure all unused indices are None.
        // Then we explicitly set scalars to 0.
        let fixed_start_idx = buf.len();
        let fixed_len = 163 - 4; // 159
        buf.resize(fixed_start_idx + fixed_len, 0xFF); // Fill with -1

        // Write known fixed fields
        // Offset 4 (relative to start_pos) -> Index 0 in fixed buffer
        // buf[fixed_start_idx + (Offset - 4)]

        // Populate Fixed Fields
        // offset 4: unknown
        buf[start_pos + 4] = if self.unknown { 1 } else { 0 };
        // offset 5: draw_type
        buf[start_pos + 5] = self.draw_type;
        // offset 6: material
        buf[start_pos + 6] = self.material;
        // offset 7: opacity
        buf[start_pos + 7] = self.opacity;

        // offset 8: hitbox (int) -> 0 to reference BlockHitboxes index 0
        {
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(0);
            let idx = start_pos + 8;
            buf[idx..idx + 4].copy_from_slice(&temp);
        }

        // offset 12: interactionHitbox -> 0 to reference BlockHitboxes index 0
        {
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(0);
            let idx = start_pos + 12;
            buf[idx..idx + 4].copy_from_slice(&temp);
        }

        // offset 16: modelScale (float)
        {
            let mut temp = BytesMut::with_capacity(4);
            temp.put_f32_le(1.0);
            let idx = start_pos + 16;
            buf[idx..idx + 4].copy_from_slice(&temp);
        }

        buf[start_pos + 20] = 0; // looping

        // offset 21: maxSupportDistance
        {
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(0);
            let idx = start_pos + 21;
            buf[idx..idx + 4].copy_from_slice(&temp);
        }

        buf[start_pos + 25] = 0; // blockSupportsRequiredFor
        buf[start_pos + 26] = 0; // requiresAlphaBlending
        buf[start_pos + 27] = 0; // cubeShadingMode
        buf[start_pos + 28] = 0; // randomRotation
        buf[start_pos + 29] = 0; // variantRotation
        buf[start_pos + 30] = 0; // rotationYawPlacementOffset

        // offset 31: blockSoundSetIndex -> 0 (BlockSoundSets IS populated)
        {
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(0); // Index 0 = Default sound set
            let idx = start_pos + 31;
            buf[idx..idx + 4].copy_from_slice(&temp);
        }

        // offset 35: ambientSoundEventIndex -> 0 (SoundEvents IS populated)
        {
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(0); // Index 0 = Default sound event
            let idx = start_pos + 35;
            buf[idx..idx + 4].copy_from_slice(&temp);
        }

        // offset 94: group
        {
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(-1);
            let idx = start_pos + 94;
            buf[idx..idx + 4].copy_from_slice(&temp);
        }

        // offset 159: transitionToTag
        {
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(-1);
            let idx = start_pos + 159;
            buf[idx..idx + 4].copy_from_slice(&temp);
        }

        // 3. Variable Offsets (24 entries * 4 bytes)
        // Initialize all to -1 (like Java does for null fields)
        let offsets_start_idx = buf.len(); // Should be start_pos + 163
        let offsets_len = 24 * 4;
        // Fill with 0xFF bytes (so each i32 reads as -1)
        buf.resize(offsets_start_idx + offsets_len, 0xFF);

        let var_data_start_idx = buf.len(); // Should be start_pos + 259

        let mut null_bits = [0u8; 4];

        // Write Item (String) - ID 0 (nullBits[0] & 1) -> Index 0
        if let Some(item_str) = &self.item {
            null_bits[0] |= 1;
            let current_pos = buf.len();
            let relative_offset = (current_pos - var_data_start_idx) as i32;
            // Write offset at index 0 (0 * 4 = 0 bytes offset from start of table)
            let off_pos = offsets_start_idx + 0;
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(relative_offset);
            let range = off_pos..off_pos + 4;
            buf[range].copy_from_slice(&temp);

            write_string(buf, item_str);
        }

        // Write Name (String) - ID 1 (nullBits[0] & 2) -> Index 1
        {
            null_bits[0] |= 2;
            let current_pos = buf.len();
            let relative_offset = (current_pos - var_data_start_idx) as i32;
            // Write offset at index 1 (1 * 4 = 4 bytes offset from start of table)
            let off_pos = offsets_start_idx + 4;
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(relative_offset);
            let range = off_pos..off_pos + 4;
            buf[range].copy_from_slice(&temp);

            write_string(buf, &self.name);
        }

        // Write CubeTextures (Array) - ID 8 (nullBits[1] & 1) -> Index 8
        if let Some(textures) = &self.cube_textures {
            null_bits[1] |= 1;
            let current_pos = buf.len();
            let relative_offset = (current_pos - var_data_start_idx) as i32;
            // index 8 * 4 = 32
            let off_pos = offsets_start_idx + 32;
            let mut temp = BytesMut::with_capacity(4);
            temp.put_i32_le(relative_offset);
            let range = off_pos..off_pos + 4;
            buf[range].copy_from_slice(&temp);

            write_varint(buf, textures.len() as i32);
            for tex in textures {
                tex.write(buf);
            }
        }

        // Write NullBits
        let nb_pos = start_pos;
        buf[nb_pos] = null_bits[0];
        buf[nb_pos + 1] = null_bits[1];
        buf[nb_pos + 2] = null_bits[2];
        buf[nb_pos + 3] = null_bits[3];
    }
}
