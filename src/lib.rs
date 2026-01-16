//! Hytale network protocol implementation.
//!
//! Provides packet encoding/decoding for Hytale server/client communication.
//! Supports all packet types used in the official Hytale client and server.
//!
//! # Example
//!
//! ```no_run
//! use hytale_protocol::packets::connection::Connect;
//! use hytale_protocol::codec::PacketRead;
//! use std::io::Cursor;
//!
//! let data: &[u8] = &[/* packet bytes */];
//! let mut cursor = Cursor::new(data);
//! let packet = Connect::read(&mut cursor).unwrap();
//! ```

pub mod block;
pub mod chunk;
pub mod chunk_data;
pub mod codec;
pub mod common;
pub mod identifier;
pub mod packet;
pub mod packets;
pub mod types;

pub use block::{BlockRegistry, BlockState, BlockStateId};
pub use chunk::{Chunk, ChunkSection};
pub use codec::{PacketRead, PacketWrite};
pub use identifier::Identifier;
pub use packets::play::ChunkDataPacket;
pub use types::{BlockPos, ChunkPos, Transform, Vec3f, Vec3i};
