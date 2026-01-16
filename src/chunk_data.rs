use bytes::{BufMut, BytesMut};
use std::collections::HashMap;

pub struct HytaleChunk {
    /// 32x32x32 blocks = 32768
    pub blocks: Vec<u8>,
}

impl HytaleChunk {
    pub fn new_flat(ground_block: u8, height: usize) -> Self {
        let mut blocks = vec![0u8; 32768];
        // Fill up to height
        for y in 0..height.min(32) {
            for z in 0..32 {
                for x in 0..32 {
                    let idx = y * 32 * 32 + z * 32 + x;
                    blocks[idx] = ground_block;
                }
            }
        }
        Self { blocks }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();

        // Check if empty
        let is_empty = self.blocks.iter().all(|&b| b == 0);

        if is_empty {
            // PaletteType::Empty (0)
            buf.put_u8(0);
        } else {
            // Use HalfByte palette (1) logic
            serialize_half_byte_palette(&mut buf, &self.blocks);
        }

        buf.put_u8(0);
        buf.put_u8(0);
        buf.to_vec()
    }
}

fn serialize_half_byte_palette(buf: &mut BytesMut, blocks: &[u8]) {
    buf.put_u8(1);

    let mut counts = HashMap::new();
    for &b in blocks {
        *counts.entry(b).or_insert(0u16) += 1;
    }

    let mut entries = Vec::new();
    let mut external_to_internal = HashMap::new();

    // Create palette entries
    // Sort by ID to be deterministic, though not required
    let mut sorted_keys: Vec<u8> = counts.keys().cloned().collect();
    sorted_keys.sort();

    // Map to internal IDs (0..15)
    for (i, &external_id) in sorted_keys.iter().enumerate() {
        if i >= 16 {
            // Fallback to Byte palette if >16 blocks (not implemented yet for this minimal usage)
            panic!("Too many block types for HalfByte palette!");
        }
        let internal_id = i as u8;
        let count = *counts.get(&external_id).unwrap();

        entries.push((internal_id, external_id, count));
        external_to_internal.insert(external_id, internal_id);
    }

    buf.put_u16_le(entries.len() as u16);

    for (internal, external, count) in entries {
        buf.put_u8(internal);
        buf.put_i32_le(external as i32); // External ID is i32
        buf.put_u16_le(count);
    }

    let mut packed_data = vec![0u8; 16384];
    for (i, &block) in blocks.iter().enumerate() {
        let internal = *external_to_internal.get(&block).unwrap();
        // Pack: even index -> low nibble, odd index -> high nibble
        let byte_idx = i / 2;
        if i % 2 == 0 {
            packed_data[byte_idx] |= internal & 0x0F;
        } else {
            packed_data[byte_idx] |= (internal & 0x0F) << 4;
        }
    }

    buf.put_slice(&packed_data);
}
