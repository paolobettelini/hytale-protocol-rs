# hytale-protocol

Rust implementation of the Hytale network protocol for server and client communication.

> [!WARNING]
> This crate is in early development. The protocol is subject to change
and has not been thoroughly tested. The protocol has been extracted
from the Hytale server `jar`.

## Usage

### Receive Server-Bound Packets

You can use the `deserialize_server_bound` function to parse raw packet data into a `ServerBoundPacket` enum, which allows you to match on specific packet types.

```rust
use hytale_protocol::{deserialize_server_bound, ServerBoundPacket};

let packet_id = 0; // Connect packet ID
let payload = vec![...]; // Raw packet data excluding length and ID

match deserialize_server_bound(packet_id, &payload) {
    Ok(ServerBoundPacket::Connect(connect_packet)) => {
        info!("Player connected: {:#?}", &connect_packet);
    },
    Ok(ServerBoundPacket::AuthToken(token_packet)) => {
        info!("Received auth token: {:#?}", &token_packet);
    },
    Err(e) => {
        error!("Failed to parse packet {}: {}", packet_id, e);
    },
    _ => {},
}
```

### Manual Packet Reading

```rust
use hytale_protocol::packets::connection::Connect;
use hytale_protocol::codec::{PacketRead, PacketBuffer};
use bytes::Bytes;

let mut buf = PacketBuffer::new(Bytes::from(payload));
let packet = Connect::read(&mut buf)?;
```

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