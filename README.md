# hytale-protocol

Rust implementation of the Hytale network protocol for server and client communication.

> [!WARNING]
> This crate is in early development. The protocol is subject to change
and has not been thoroughly tested. The protocol has been extracted
from the Hytale server `jar`.

## Usage

TODO

## Types

### Core Types
- `Identifier`: Namespaced identifiers (`namespace:path`)
- `BlockPos`, `ChunkPos`: Position types
- `Vec3f`, `Vec3i`: 3D vectors
- `Transform`: Position + rotation

### Packet Traits
- `Packet`: Defines packet ID and compression
- `PacketRead`: Deserialize from bytes
- `PacketWrite`: Serialize to bytes