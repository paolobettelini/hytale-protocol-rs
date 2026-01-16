use crate::chunk::Chunk;
use crate::codec::{
    CodecResult, PacketBuffer, PacketRead, PacketWrite, write_string, write_varint,
};
use crate::packet::{Packet, PacketDirection, PacketId};
use bytes::BufMut;

/// Join game packet sent when player enters the world.
#[derive(Debug, Clone)]
pub struct JoinGamePacket {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub dimension_names: Vec<String>,
    pub dimension_type: String,
    pub dimension_name: String,
    pub hashed_seed: i64,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}

impl PacketRead for JoinGamePacket {
    fn read(_buf: &mut PacketBuffer) -> CodecResult<Self> {
        // Simplified - in a real implementation we'd read all fields
        unimplemented!("JoinGamePacket read not implemented")
    }
}

impl PacketWrite for JoinGamePacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        buf.put_i32(self.entity_id);
        buf.put_u8(if self.is_hardcore { 1 } else { 0 });
        buf.put_u8(self.game_mode);
        buf.put_i8(self.previous_game_mode);

        // Dimension names
        write_varint(buf, self.dimension_names.len() as i32);
        for name in &self.dimension_names {
            write_string(buf, name);
        }

        // Registry codec (simplified - empty NBT)
        buf.put_u8(0x0a); // Compound tag
        buf.put_u16(0); // Empty name
        buf.put_u8(0); // End tag

        write_string(buf, &self.dimension_type);
        write_string(buf, &self.dimension_name);
        buf.put_i64(self.hashed_seed);
        write_varint(buf, self.max_players);
        write_varint(buf, self.view_distance);
        write_varint(buf, self.simulation_distance);
        buf.put_u8(if self.reduced_debug_info { 1 } else { 0 });
        buf.put_u8(if self.enable_respawn_screen { 1 } else { 0 });
        buf.put_u8(if self.is_debug { 1 } else { 0 });
        buf.put_u8(if self.is_flat { 1 } else { 0 });
        buf.put_u8(0); // Has death location = false
    }
}

impl Packet for JoinGamePacket {
    const ID: PacketId = 0x28;
    const DIRECTION: PacketDirection = PacketDirection::Clientbound;
}

/// Chunk data packet.
#[derive(Debug, Clone)]
pub struct ChunkDataPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: bytes::Bytes,
}

impl ChunkDataPacket {
    /// Create a chunk data packet from a chunk.
    pub fn from_chunk(chunk: &Chunk) -> Self {
        let mut buf = bytes::BytesMut::new();
        chunk.write_to(&mut buf);
        Self {
            chunk_x: chunk.x,
            chunk_z: chunk.z,
            data: buf.freeze(),
        }
    }
}

impl PacketRead for ChunkDataPacket {
    fn read(_buf: &mut PacketBuffer) -> CodecResult<Self> {
        unimplemented!("ChunkDataPacket read not implemented")
    }
}

impl PacketWrite for ChunkDataPacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        buf.put_i32(self.chunk_x);
        buf.put_i32(self.chunk_z);
        buf.extend_from_slice(&self.data);
    }
}

impl Packet for ChunkDataPacket {
    const ID: PacketId = 0x24;
    const DIRECTION: PacketDirection = PacketDirection::Clientbound;
}

/// Player position and look (server to client).
#[derive(Debug, Clone)]
pub struct PlayerPositionLookPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: i32,
}

impl PacketRead for PlayerPositionLookPacket {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            x: buf.read_f64()?,
            y: buf.read_f64()?,
            z: buf.read_f64()?,
            yaw: buf.read_f32()?,
            pitch: buf.read_f32()?,
            flags: buf.read_u8()?,
            teleport_id: buf.read_varint()?,
        })
    }
}

impl PacketWrite for PlayerPositionLookPacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        buf.put_f64(self.x);
        buf.put_f64(self.y);
        buf.put_f64(self.z);
        buf.put_f32(self.yaw);
        buf.put_f32(self.pitch);
        buf.put_u8(self.flags);
        write_varint(buf, self.teleport_id);
    }
}

impl Packet for PlayerPositionLookPacket {
    const ID: PacketId = 0x3C;
    const DIRECTION: PacketDirection = PacketDirection::Clientbound;
}

/// Keep alive packet.
#[derive(Debug, Clone)]
pub struct KeepAlivePacket {
    pub id: i64,
}

impl PacketRead for KeepAlivePacket {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            id: buf.read_i64()?,
        })
    }
}

impl PacketWrite for KeepAlivePacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        buf.put_i64(self.id);
    }
}

impl Packet for KeepAlivePacket {
    const ID: PacketId = 0x23;
    const DIRECTION: PacketDirection = PacketDirection::Clientbound;
}

#[derive(Debug, Clone)]
pub struct DisconnectPacket {
    pub reason: String,
}

impl PacketRead for DisconnectPacket {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        Ok(Self {
            reason: buf.read_string()?,
        })
    }
}

impl PacketWrite for DisconnectPacket {
    fn write(&self, buf: &mut bytes::BytesMut) {
        // Write as JSON text component
        let json = format!(r#"{{"text":"{}"}}"#, self.reason);
        write_string(buf, &json);
    }
}

impl Packet for DisconnectPacket {
    const ID: PacketId = 0x1A;
    const DIRECTION: PacketDirection = PacketDirection::Clientbound;
}
