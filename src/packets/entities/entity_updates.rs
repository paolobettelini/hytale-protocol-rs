use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_varint};
use bytes::{BufMut, BytesMut};

/// EntityUpdates packet (ID 161) - Large entity updates / spawning
#[derive(Debug, Clone)]
pub struct EntityUpdates {
    pub removed_entities: Vec<i32>,
    pub updates: Vec<EntityUpdate>,
}

#[derive(Debug, Clone)]
pub struct EntityUpdate {
    pub network_id: i32,
    pub removed_components: Vec<u8>,
    pub updated_components: Vec<u8>, // Raw component data for now
}

impl Packet for EntityUpdates {
    const PACKET_ID: u32 = 161;
    fn is_compressed() -> bool { true }
}

impl PacketRead for EntityUpdates {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        // Offsets
        buf.read_int_le()?;
        buf.read_int_le()?;
        
        let mut removed_entities = Vec::new();
        if (null_bits & 1) != 0 {
            let count = buf.read_varint()? as usize;
            for _ in 0..count {
                removed_entities.push(buf.read_int_le()?);
            }
        }
        
        let mut updates = Vec::new();
        if (null_bits & 2) != 0 {
            let count = buf.read_varint()? as usize;
            for _ in 0..count {
                let u_null_bits = buf.read_u8()?;
                let network_id = buf.read_int_le()?;
                buf.read_int_le()?; // removed offset
                buf.read_int_le()?; // updates offset
                
                let mut removed_components = Vec::new();
                if (u_null_bits & 1) != 0 {
                    let c_count = buf.read_varint()? as usize;
                    for _ in 0..c_count {
                        removed_components.push(buf.read_u8()?);
                    }
                }
                
                let mut updated_components = Vec::new();
                if (u_null_bits & 2) != 0 {
                    // For now, we don't fully parse components, just read remaining or next VarInt length
                    // In real Hytale, this is very complex.
                    let c_count = buf.read_varint()? as usize;
                    // Skip for now or read as raw bytes if possible
                }
                
                updates.push(EntityUpdate {
                    network_id,
                    removed_components,
                    updated_components,
                });
            }
        }
        
        Ok(Self { removed_entities, updates })
    }
}

impl PacketWrite for EntityUpdates {
    fn write(&self, buf: &mut BytesMut) {
        let mut null_bits = 0u8;
        if !self.removed_entities.is_empty() { null_bits |= 1; }
        if !self.updates.is_empty() { null_bits |= 2; }
        buf.put_u8(null_bits);

        let mut field_data = BytesMut::new();
        let mut offsets = [0i32; 2];

        if !self.removed_entities.is_empty() {
            offsets[0] = field_data.len() as i32;
            write_varint(&mut field_data, self.removed_entities.len() as i32);
            for id in &self.removed_entities {
                field_data.put_i32_le(*id);
            }
        } else {
            offsets[0] = -1;
        }

        if !self.updates.is_empty() {
            offsets[1] = field_data.len() as i32;
            write_varint(&mut field_data, self.updates.len() as i32);
            for update in &self.updates {
                let mut u_null_bits = 0u8;
                if !update.removed_components.is_empty() { u_null_bits |= 1; }
                if !update.updated_components.is_empty() { u_null_bits |= 2; }
                field_data.put_u8(u_null_bits);
                field_data.put_i32_le(update.network_id);
                
                let mut u_field_data = BytesMut::new();
                let mut u_offsets = [0i32; 2];
                
                if !update.removed_components.is_empty() {
                    u_offsets[0] = u_field_data.len() as i32;
                    write_varint(&mut u_field_data, update.removed_components.len() as i32);
                    u_field_data.put_slice(&update.removed_components);
                } else {
                    u_offsets[0] = -1;
                }
                
                if !update.updated_components.is_empty() {
                    u_offsets[1] = u_field_data.len() as i32;
                    write_varint(&mut u_field_data, 1); // Mock 1 component
                    u_field_data.put_slice(&update.updated_components);
                } else {
                    u_offsets[1] = -1;
                }
                
                field_data.put_i32_le(u_offsets[0]);
                field_data.put_i32_le(u_offsets[1]);
                field_data.put_slice(&u_field_data);
            }
        } else {
            offsets[1] = -1;
        }

        buf.put_i32_le(offsets[0]);
        buf.put_i32_le(offsets[1]);
        buf.put_slice(&field_data);
    }
}
