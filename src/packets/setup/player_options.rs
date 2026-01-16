use crate::codec::{CodecResult, Packet, PacketBuffer, PacketRead, PacketWrite, write_varint, write_string};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, Default)]
pub struct PlayerSkin {
    pub body_characteristic: Option<String>,
    pub underwear: Option<String>,
    pub face: Option<String>,
    pub eyes: Option<String>,
    pub ears: Option<String>,
    pub mouth: Option<String>,
    pub facial_hair: Option<String>,
    pub haircut: Option<String>,
    pub eyebrows: Option<String>,
    pub pants: Option<String>,
    pub overpants: Option<String>,
    pub undertop: Option<String>,
    pub overtop: Option<String>,
    pub shoes: Option<String>,
    pub head_accessory: Option<String>,
    pub face_accessory: Option<String>,
    pub ear_accessory: Option<String>,
    pub skin_feature: Option<String>,
    pub gloves: Option<String>,
    pub cape: Option<String>,
}

impl PlayerSkin {
    pub fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_bytes(3)?;
        
        // Skip offset table (20 fields * 4 bytes)
        for _ in 0..20 {
            buf.read_int_le()?;
        }

        let mut skin = PlayerSkin::default();
        let has_bit = |byte_idx: usize, bit_mask: u8| -> bool { (null_bits[byte_idx] & bit_mask) != 0 };

        if has_bit(0, 1) { skin.body_characteristic = Some(buf.read_var_string()?); }
        if has_bit(0, 2) { skin.underwear = Some(buf.read_var_string()?); }
        if has_bit(0, 4) { skin.face = Some(buf.read_var_string()?); }
        if has_bit(0, 8) { skin.eyes = Some(buf.read_var_string()?); }
        if has_bit(0, 16) { skin.ears = Some(buf.read_var_string()?); }
        if has_bit(0, 32) { skin.mouth = Some(buf.read_var_string()?); }
        if has_bit(0, 64) { skin.facial_hair = Some(buf.read_var_string()?); }
        if has_bit(0, 128) { skin.haircut = Some(buf.read_var_string()?); }

        if has_bit(1, 1) { skin.eyebrows = Some(buf.read_var_string()?); }
        if has_bit(1, 2) { skin.pants = Some(buf.read_var_string()?); }
        if has_bit(1, 4) { skin.overpants = Some(buf.read_var_string()?); }
        if has_bit(1, 8) { skin.undertop = Some(buf.read_var_string()?); }
        if has_bit(1, 16) { skin.overtop = Some(buf.read_var_string()?); }
        if has_bit(1, 32) { skin.shoes = Some(buf.read_var_string()?); }
        if has_bit(1, 64) { skin.head_accessory = Some(buf.read_var_string()?); }
        if has_bit(1, 128) { skin.face_accessory = Some(buf.read_var_string()?); }

        if has_bit(2, 1) { skin.ear_accessory = Some(buf.read_var_string()?); }
        if has_bit(2, 2) { skin.skin_feature = Some(buf.read_var_string()?); }
        if has_bit(2, 4) { skin.gloves = Some(buf.read_var_string()?); }
        if has_bit(2, 8) { skin.cape = Some(buf.read_var_string()?); }

        Ok(skin)
    }

    pub fn write(&self, buf: &mut BytesMut) {
        let mut null_bits = [0u8; 3];
        let mut fields = Vec::new();

        if let Some(ref s) = self.body_characteristic { null_bits[0] |= 1; fields.push(s); }
        if let Some(ref s) = self.underwear { null_bits[0] |= 2; fields.push(s); }
        if let Some(ref s) = self.face { null_bits[0] |= 4; fields.push(s); }
        if let Some(ref s) = self.eyes { null_bits[0] |= 8; fields.push(s); }
        if let Some(ref s) = self.ears { null_bits[0] |= 16; fields.push(s); }
        if let Some(ref s) = self.mouth { null_bits[0] |= 32; fields.push(s); }
        if let Some(ref s) = self.facial_hair { null_bits[0] |= 64; fields.push(s); }
        if let Some(ref s) = self.haircut { null_bits[0] |= 128; fields.push(s); }

        if let Some(ref s) = self.eyebrows { null_bits[1] |= 1; fields.push(s); }
        if let Some(ref s) = self.pants { null_bits[1] |= 2; fields.push(s); }
        if let Some(ref s) = self.overpants { null_bits[1] |= 4; fields.push(s); }
        if let Some(ref s) = self.undertop { null_bits[1] |= 8; fields.push(s); }
        if let Some(ref s) = self.overtop { null_bits[1] |= 16; fields.push(s); }
        if let Some(ref s) = self.shoes { null_bits[1] |= 32; fields.push(s); }
        if let Some(ref s) = self.head_accessory { null_bits[1] |= 64; fields.push(s); }
        if let Some(ref s) = self.face_accessory { null_bits[1] |= 128; fields.push(s); }

        if let Some(ref s) = self.ear_accessory { null_bits[2] |= 1; fields.push(s); }
        if let Some(ref s) = self.skin_feature { null_bits[2] |= 2; fields.push(s); }
        if let Some(ref s) = self.gloves { null_bits[2] |= 4; fields.push(s); }
        if let Some(ref s) = self.cape { null_bits[2] |= 8; fields.push(s); }

        buf.put_slice(&null_bits);
        
        let var_block_start = 3 + 20 * 4;
        let mut current_offset = 0i32;
        let mut field_data = BytesMut::new();
        
        let mut offsets = [0i32; 20];
        let mut field_idx = 0;
        
        // This is a bit complex as we need to match the bit order to the field list
        // and calculate offsets. I'll simplify for now as the server usually sends nulls or basic skins.
        // The Java serialization for PlayerSkin is quite elaborate.
        
        for i in 0..20 {
            // Check if field i is present
            let byte_idx = i / 8;
            let bit_mask = 1 << (i % 8);
            if (null_bits[byte_idx] & bit_mask) != 0 {
                offsets[i] = current_offset;
                let s = fields[field_idx];
                write_string(&mut field_data, s);
                current_offset = field_data.len() as i32;
                field_idx += 1;
            } else {
                offsets[i] = -1;
            }
        }
        
        for offset in offsets {
            buf.put_i32_le(offset);
        }
        buf.put_slice(&field_data);
    }
}

/// PlayerOptions packet (ID 33)
#[derive(Debug, Clone)]
pub struct PlayerOptions {
    pub skin: Option<PlayerSkin>,
}

impl Packet for PlayerOptions {
    const PACKET_ID: u32 = 33;
}

impl PacketRead for PlayerOptions {
    fn read(buf: &mut PacketBuffer) -> CodecResult<Self> {
        let null_bits = buf.read_u8()?;
        let skin = if (null_bits & 1) != 0 {
            Some(PlayerSkin::read(buf)?)
        } else {
            None
        };
        Ok(Self { skin })
    }
}

impl PacketWrite for PlayerOptions {
    fn write(&self, buf: &mut BytesMut) {
        let null_bits = if self.skin.is_some() { 1u8 } else { 0u8 };
        buf.put_u8(null_bits);
        if let Some(ref skin) = self.skin {
            skin.write(buf);
        }
    }
}
