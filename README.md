# hytale-protocol

Rust implementation of the Hytale network protocol for server and client communication.

> [!WARNING]
> This crate is in early development. The protocol is subject to change
and has not been thoroughly tested. The protocol has been extracted
from the Hytale server `jar`.

## Usage

### Reading Packets

```rust
use hytale_protocol::packets::connection::Connect;
use hytale_protocol::codec::PacketRead;
use std::io::Cursor;

let data: &[u8] = /* packet bytes */;
let mut cursor = Cursor::new(data);
let packet = Connect::read(&mut cursor)?;

println!("Player {} connected", packet.username);
```

### Writing Packets

```rust
use hytale_protocol::packets::connection::Connect;
use hytale_protocol::codec::{Packet, PacketWrite};
use bytes::BytesMut;
use uuid::Uuid;

let packet = Connect {
    protocol_hash: "abc123".to_string(),
    client_type: ClientType::Game,
    uuid: Uuid::new_v4(),
    username: "Player1".to_string(),
    ..Default::default()
};

let mut buf = BytesMut::new();
packet.write(&mut buf)?;

send_to_client(Connect::ID, &buf);
```

### Packet Encoding

```rust
use hytale_protocol::codec;

let frame = codec::encode_raw_packet(
    packet_id,
    compressed,
    &packet_data
)?;
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