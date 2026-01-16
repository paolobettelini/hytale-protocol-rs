use bytes::{Buf, BufMut, Bytes, BytesMut};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("Buffer underflow: expected {expected} bytes, got {available}")]
    BufferUnderflow { expected: usize, available: usize },
    #[error("Invalid string length: {0}")]
    InvalidStringLength(usize),
    #[error("Invalid enum variant: {0}")]
    InvalidEnumVariant(i32),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Codec error: {0}")]
    Other(String),
}

pub type CodecResult<T> = Result<T, CodecError>;

/// Trait for packets with an ID.
pub trait Packet {
    const PACKET_ID: u32;

    /// Whether the packet payload should be Zstd compressed.
    fn is_compressed() -> bool {
        false
    }
}

/// Trait for reading packets from a buffer.
pub trait PacketRead: Sized {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self>;
}

/// Trait for writing packets to a buffer.
pub trait PacketWrite {
    fn write(&self, buf: &mut BytesMut);
}

/// Buffer wrapper for reading packet data.
pub struct PacketBuffer {
    data: Bytes,
}

impl PacketBuffer {
    pub fn new(data: Bytes) -> Self {
        Self { data }
    }

    pub fn remaining(&self) -> usize {
        self.data.remaining()
    }

    pub fn read_u8(&mut self) -> CodecResult<u8> {
        if self.data.remaining() < 1 {
            return Err(CodecError::BufferUnderflow {
                expected: 1,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_u8())
    }

    pub fn read_byte(&mut self) -> CodecResult<u8> {
        self.read_u8()
    }

    pub fn read_i8(&mut self) -> CodecResult<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_u16(&mut self) -> CodecResult<u16> {
        if self.data.remaining() < 2 {
            return Err(CodecError::BufferUnderflow {
                expected: 2,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_u16())
    }

    pub fn read_i16(&mut self) -> CodecResult<i16> {
        Ok(self.read_u16()? as i16)
    }

    pub fn read_u32(&mut self) -> CodecResult<u32> {
        if self.data.remaining() < 4 {
            return Err(CodecError::BufferUnderflow {
                expected: 4,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_u32())
    }

    pub fn read_i32(&mut self) -> CodecResult<i32> {
        Ok(self.read_u32()? as i32)
    }

    pub fn read_int_le(&mut self) -> CodecResult<i32> {
        if self.data.remaining() < 4 {
            return Err(CodecError::BufferUnderflow {
                expected: 4,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_i32_le())
    }

    pub fn read_u64(&mut self) -> CodecResult<u64> {
        if self.data.remaining() < 8 {
            return Err(CodecError::BufferUnderflow {
                expected: 8,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_u64())
    }

    pub fn read_i64(&mut self) -> CodecResult<i64> {
        Ok(self.read_u64()? as i64)
    }

    pub fn read_f32(&mut self) -> CodecResult<f32> {
        if self.data.remaining() < 4 {
            return Err(CodecError::BufferUnderflow {
                expected: 4,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_f32())
    }

    pub fn read_f64(&mut self) -> CodecResult<f64> {
        if self.data.remaining() < 8 {
            return Err(CodecError::BufferUnderflow {
                expected: 8,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_f64())
    }

    pub fn read_bool(&mut self) -> CodecResult<bool> {
        Ok(self.read_u8()? != 0)
    }

    /// Read a variable-length integer (VarInt).
    pub fn read_varint(&mut self) -> CodecResult<i32> {
        let mut value: i32 = 0;
        let mut position: u32 = 0;

        loop {
            let byte = self.read_u8()?;
            value |= ((byte & 0x7F) as i32) << position;

            if (byte & 0x80) == 0 {
                break;
            }

            position += 7;
            if position >= 32 {
                return Err(CodecError::InvalidStringLength(position as usize));
            }
        }

        Ok(value)
    }

    pub fn read_var_int(&mut self) -> CodecResult<i32> {
        self.read_varint()
    }

    pub fn read_string(&mut self) -> CodecResult<String> {
        let len = self.read_varint()? as usize;
        if len > 32767 {
            return Err(CodecError::InvalidStringLength(len));
        }
        if self.data.remaining() < len {
            return Err(CodecError::BufferUnderflow {
                expected: len,
                available: self.data.remaining(),
            });
        }
        let bytes = self.data.copy_to_bytes(len);
        String::from_utf8(bytes.to_vec()).map_err(CodecError::Utf8)
    }

    pub fn read_var_string(&mut self) -> CodecResult<String> {
        self.read_string()
    }

    pub fn read_bytes(&mut self, len: usize) -> CodecResult<Bytes> {
        if self.data.remaining() < len {
            return Err(CodecError::BufferUnderflow {
                expected: len,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.copy_to_bytes(len))
    }
}

// Write helpers
pub fn write_varint(buf: &mut BytesMut, mut value: i32) {
    loop {
        if (value & !0x7F) == 0 {
            buf.put_u8(value as u8);
            return;
        }
        buf.put_u8(((value & 0x7F) | 0x80) as u8);
        value = ((value as u32) >> 7) as i32;
    }
}

pub fn write_string(buf: &mut BytesMut, s: &str) {
    write_varint(buf, s.len() as i32);
    buf.put_slice(s.as_bytes());
}

pub fn var_int_size(val: i32) -> i32 {
    let mut x = val as u32;
    if (x & 0xFFFFFF80) == 0 {
        1
    } else if (x & 0xFFFFC000) == 0 {
        2
    } else if (x & 0xFFE00000) == 0 {
        3
    } else if (x & 0xF0000000) == 0 {
        4
    } else {
        5
    }
}

/// Encode a packet into a complete frame (Length + ID + Payload).
pub fn encode_packet<P: Packet + PacketWrite>(packet: &P) -> CodecResult<BytesMut> {
    let mut payload_buf = BytesMut::new();
    packet.write(&mut payload_buf);

    let payload = if P::is_compressed() && !payload_buf.is_empty() {
        // Use bulk compress to ensure content size is written to the frame header,
        // which matches Java's Zstd.compress behavior and expectation.
        let compressed = zstd::bulk::compress(&payload_buf, 0).map_err(CodecError::Io)?;
        BytesMut::from(&compressed[..])
    } else {
        payload_buf
    };

    let mut frame = BytesMut::with_capacity(8 + payload.len());
    frame.put_u32_le(payload.len() as u32);
    frame.put_u32_le(P::PACKET_ID);
    frame.put(payload);

    Ok(frame)
}

/// Decode a packet payload (excluding Length and ID).
/// Handles decompression if needed.
pub fn decode_payload<P: Packet + PacketRead>(payload: &[u8]) -> CodecResult<P> {
    let data = if P::is_compressed() && !payload.is_empty() {
        // Use bulk decompress to match the encoding.
        // We use a generous limit (50MB) for decompression since we don't have per-packet MAX_SIZE in the trait yet.
        // Hytale packets can be large (up to ~32MB for ServerInfo, larger for WorldSettings).
        const DECOMPRESSION_LIMIT: usize = 50 * 1024 * 1024;
        zstd::bulk::decompress(payload, DECOMPRESSION_LIMIT).map_err(CodecError::Io)?
    } else {
        payload.to_vec()
    };

    let mut buf = PacketBuffer::new(Bytes::from(data));
    P::read(&mut buf)
}

/// Encodes a raw packet from ID and payload.
/// Handles compression if is_compressed is true.
/// Uses 4-byte LE integers for length and ID to match Java protocol.
pub fn encode_raw_packet(
    packet_id: u32,
    is_compressed: bool,
    payload: &[u8],
) -> CodecResult<Bytes> {
    // Match the behavior of encode_packet exactly
    let final_payload = if is_compressed && !payload.is_empty() {
        // Use bulk compress with level 0 to ensure content size is written to the frame header,
        // which matches Java's Zstd.compress behavior and expectation.
        let compressed = zstd::bulk::compress(payload, 0).map_err(CodecError::Io)?;
        BytesMut::from(&compressed[..])
    } else {
        BytesMut::from(payload)
    };

    // Now wrap in the frame structure: Length (4 bytes LE) + ID (4 bytes LE) + Data
    let mut frame = BytesMut::with_capacity(8 + final_payload.len());

    // Write Length (4 bytes LE) - just the payload size
    frame.put_u32_le(final_payload.len() as u32);
    // Write ID (4 bytes LE)
    frame.put_u32_le(packet_id);
    // Write Payload
    frame.put(final_payload);

    Ok(frame.freeze())
}
