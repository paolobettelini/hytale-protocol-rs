use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_string};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChatType {
    Normal = 0,
    System = 1,
    Whisper = 2,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub message: String,
    pub chat_type: ChatType,
    pub sender_name: Option<String>,
}

impl Packet for ChatMessage {
    const PACKET_ID: u32 = 99;
}

impl PacketRead for ChatMessage {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        let chat_type = match buf.read_u8()? {
            1 => ChatType::System,
            2 => ChatType::Whisper,
            _ => ChatType::Normal,
        };

        let message = buf.read_var_string()?;
        let sender_name = if null_bits & 1 != 0 {
            Some(buf.read_var_string()?)
        } else {
            None
        };

        Ok(Self {
            message,
            chat_type,
            sender_name,
        })
    }
}

impl PacketWrite for ChatMessage {
    fn write(&self, buf: &mut BytesMut) {
        let null_bits = if self.sender_name.is_some() { 1u8 } else { 0u8 };
        buf.put_u8(null_bits);
        buf.put_u8(self.chat_type as u8);
        write_string(buf, &self.message);
        if let Some(ref sender) = self.sender_name {
            write_string(buf, sender);
        }
    }
}
