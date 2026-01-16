//! Chunk data structures.

use crate::block::BlockStateId;
use bytes::{BufMut, BytesMut};

/// Chunk size in blocks (16x16).
pub const CHUNK_SIZE: usize = 16;

/// Section height in blocks.
pub const SECTION_HEIGHT: usize = 16;

/// Total world height in sections.
pub const WORLD_HEIGHT_SECTIONS: usize = 24; // 384 blocks / 16

/// Minimum Y coordinate (blocks).
pub const MIN_Y: i32 = -64;

/// Maximum Y coordinate (blocks).
pub const MAX_Y: i32 = 319;

/// A single 16x16x16 chunk section.
#[derive(Clone)]
pub struct ChunkSection {
    /// Block states stored as palette index (for now, direct block IDs).
    /// Layout: [y][z][x] for cache efficiency during iteration.
    blocks: Box<[BlockStateId; CHUNK_SIZE * CHUNK_SIZE * SECTION_HEIGHT]>,
    /// Number of non-air blocks.
    non_air_count: u16,
}

impl ChunkSection {
    /// Creates an empty (all air) section.
    pub fn new() -> Self {
        Self {
            blocks: Box::new([0; CHUNK_SIZE * CHUNK_SIZE * SECTION_HEIGHT]),
            non_air_count: 0,
        }
    }

    /// Creates a section filled with a single block type.
    pub fn filled(block_id: BlockStateId) -> Self {
        let blocks = Box::new([block_id; CHUNK_SIZE * CHUNK_SIZE * SECTION_HEIGHT]);
        let non_air_count = if block_id == 0 {
            0
        } else {
            (CHUNK_SIZE * CHUNK_SIZE * SECTION_HEIGHT) as u16
        };
        Self {
            blocks,
            non_air_count,
        }
    }

    #[inline]
    fn index(x: u8, y: u8, z: u8) -> usize {
        (y as usize) * CHUNK_SIZE * CHUNK_SIZE + (z as usize) * CHUNK_SIZE + (x as usize)
    }

    /// Get the block at local coordinates.
    pub fn get(&self, x: u8, y: u8, z: u8) -> BlockStateId {
        self.blocks[Self::index(x, y, z)]
    }

    /// Set the block at local coordinates.
    pub fn set(&mut self, x: u8, y: u8, z: u8, block_id: BlockStateId) {
        let idx = Self::index(x, y, z);
        let old = self.blocks[idx];

        if old == 0 && block_id != 0 {
            self.non_air_count += 1;
        } else if old != 0 && block_id == 0 {
            self.non_air_count -= 1;
        }

        self.blocks[idx] = block_id;
    }

    /// Returns true if this section is entirely air.
    pub fn is_empty(&self) -> bool {
        self.non_air_count == 0
    }

    /// Serialize the section for network transmission.
    pub fn write_to(&self, buf: &mut BytesMut) {
        // Write block count
        buf.put_i16(self.non_air_count as i16);

        // For simplicity, use direct palette (bits per block = ceil(log2(max_block_id)))
        // In a real implementation, you'd use a proper palette system
        let bits_per_block: u8 = 15; // Direct storage
        buf.put_u8(bits_per_block);

        // No palette for direct storage
        crate::codec::write_varint(buf, 0);

        // Write block data as long array
        let longs_needed =
            (CHUNK_SIZE * CHUNK_SIZE * SECTION_HEIGHT * bits_per_block as usize + 63) / 64;
        crate::codec::write_varint(buf, longs_needed as i32);

        // Pack blocks into longs
        let mut current_long: u64 = 0;
        let mut bits_used: u8 = 0;

        for y in 0..SECTION_HEIGHT {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let block = self.blocks[y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x];
                    current_long |= (block as u64 & ((1u64 << bits_per_block) - 1)) << bits_used;
                    bits_used += bits_per_block;

                    if bits_used >= 64 {
                        buf.put_u64(current_long);
                        bits_used -= 64;
                        current_long = if bits_used > 0 {
                            (block as u64) >> (bits_per_block - bits_used)
                        } else {
                            0
                        };
                    }
                }
            }
        }

        // Write remaining bits
        if bits_used > 0 {
            buf.put_u64(current_long);
        }
    }
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self::new()
    }
}

/// A full chunk column.
pub struct Chunk {
    /// Chunk X coordinate.
    pub x: i32,
    /// Chunk Z coordinate.
    pub z: i32,
    /// Sections from bottom to top.
    sections: Vec<Option<ChunkSection>>,
}

impl Chunk {
    /// Creates a new empty chunk.
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            sections: (0..WORLD_HEIGHT_SECTIONS).map(|_| None).collect(),
        }
    }

    /// Creates a flat world chunk with the specified ground height.
    pub fn flat(x: i32, z: i32, ground_block: BlockStateId, ground_height: i32) -> Self {
        let mut chunk = Self::new(x, z);

        // Calculate which sections need blocks
        let min_section = 0; // Section containing Y=-64
        let ground_section = ((ground_height - MIN_Y) / SECTION_HEIGHT as i32) as usize;

        for section_idx in min_section..=ground_section.min(WORLD_HEIGHT_SECTIONS - 1) {
            let section_bottom_y = MIN_Y + (section_idx as i32 * SECTION_HEIGHT as i32);

            let mut section = ChunkSection::new();

            for local_y in 0..SECTION_HEIGHT {
                let world_y = section_bottom_y + local_y as i32;
                if world_y < ground_height {
                    // Fill entire layer with ground block
                    for z in 0..CHUNK_SIZE {
                        for x in 0..CHUNK_SIZE {
                            section.set(x as u8, local_y as u8, z as u8, ground_block);
                        }
                    }
                }
            }

            if !section.is_empty() {
                chunk.sections[section_idx] = Some(section);
            }
        }

        chunk
    }

    /// Get a block at world coordinates within this chunk.
    pub fn get_block(&self, x: u8, y: i32, z: u8) -> BlockStateId {
        let section_idx = ((y - MIN_Y) / SECTION_HEIGHT as i32) as usize;
        if section_idx >= WORLD_HEIGHT_SECTIONS {
            return 0;
        }

        let local_y = ((y - MIN_Y) % SECTION_HEIGHT as i32) as u8;
        self.sections[section_idx]
            .as_ref()
            .map(|s| s.get(x, local_y, z))
            .unwrap_or(0)
    }

    /// Set a block at world coordinates within this chunk.
    pub fn set_block(&mut self, x: u8, y: i32, z: u8, block_id: BlockStateId) {
        let section_idx = ((y - MIN_Y) / SECTION_HEIGHT as i32) as usize;
        if section_idx >= WORLD_HEIGHT_SECTIONS {
            return;
        }

        let local_y = ((y - MIN_Y) % SECTION_HEIGHT as i32) as u8;

        // Create section if it doesn't exist
        if self.sections[section_idx].is_none() {
            self.sections[section_idx] = Some(ChunkSection::new());
        }

        if let Some(section) = &mut self.sections[section_idx] {
            section.set(x, local_y, z, block_id);
        }
    }

    /// Serialize the chunk for network transmission.
    pub fn write_to(&self, buf: &mut BytesMut) {
        // Write chunk position
        buf.put_i32(self.x);
        buf.put_i32(self.z);

        // Build section bitmask
        let mut section_mask: u32 = 0;
        for (idx, section) in self.sections.iter().enumerate() {
            if section.is_some() {
                section_mask |= 1 << idx;
            }
        }
        buf.put_u32(section_mask);

        // Write heightmap (simplified - just zeros)
        crate::codec::write_varint(buf, 0); // Empty NBT compound

        // Write section data
        let mut section_data = BytesMut::new();
        for section in &self.sections {
            if let Some(s) = section {
                s.write_to(&mut section_data);
            }
        }

        crate::codec::write_varint(buf, section_data.len() as i32);
        buf.extend_from_slice(&section_data);

        // Write block entities (none for now)
        crate::codec::write_varint(buf, 0);
    }
}
