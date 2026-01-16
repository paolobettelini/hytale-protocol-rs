use crate::codec::{CodecError, CodecResult, Packet, decode_payload};
use crate::packets::auth::AuthToken;
use crate::packets::connection::{Connect, Disconnect, Ping};
use crate::packets::interface::{BlockChange, ChatMessage};
use crate::packets::inventory::{DropItemStack, MoveItemStack, SetActiveSlot};
use crate::packets::player::{ClientMovement, ClientReady, ClientTeleport};
use crate::packets::setup::{PlayerOptions, RequestAssets, RequestCommonAssetsRebuild};

#[derive(Debug, Clone)]
pub enum ServerBoundPacket {
    Connect(Connect),
    Disconnect(Disconnect),
    Ping(Ping),
    AuthToken(AuthToken),
    RequestAssets(RequestAssets),
    ClientReady(ClientReady),
    ClientMovement(ClientMovement),
    ClientTeleport(ClientTeleport),
    PlayerOptions(PlayerOptions),
    RequestCommonAssetsRebuild(RequestCommonAssetsRebuild),
    ChatMessage(ChatMessage),
    BlockChange(BlockChange),
    SetActiveSlot(SetActiveSlot),
    DropItemStack(DropItemStack),
    MoveItemStack(MoveItemStack),
    // ...
}

/// Deserialize a packet based on its ID and payload.
/// 
/// The payload should be the raw packet data (excluding length and ID).
/// This function handles decompression if the packet type requires it.
pub fn deserialize_server_bound(packet_id: u32, payload: &[u8]) -> CodecResult<ServerBoundPacket> {
    match packet_id {
        Connect::PACKET_ID => Ok(ServerBoundPacket::Connect(decode_payload(payload)?)),
        Disconnect::PACKET_ID => Ok(ServerBoundPacket::Disconnect(decode_payload(payload)?)),
        Ping::PACKET_ID => Ok(ServerBoundPacket::Ping(decode_payload(payload)?)),
        AuthToken::PACKET_ID => Ok(ServerBoundPacket::AuthToken(decode_payload(payload)?)),
        RequestAssets::PACKET_ID => Ok(ServerBoundPacket::RequestAssets(decode_payload(payload)?)),
        ClientReady::PACKET_ID => Ok(ServerBoundPacket::ClientReady(decode_payload(payload)?)),
        ClientMovement::PACKET_ID => Ok(ServerBoundPacket::ClientMovement(decode_payload(payload)?)),
        ClientTeleport::PACKET_ID => Ok(ServerBoundPacket::ClientTeleport(decode_payload(payload)?)),
        PlayerOptions::PACKET_ID => Ok(ServerBoundPacket::PlayerOptions(decode_payload(payload)?)),
        RequestCommonAssetsRebuild::PACKET_ID => Ok(ServerBoundPacket::RequestCommonAssetsRebuild(decode_payload(payload)?)),
        ChatMessage::PACKET_ID => Ok(ServerBoundPacket::ChatMessage(decode_payload(payload)?)),
        BlockChange::PACKET_ID => Ok(ServerBoundPacket::BlockChange(decode_payload(payload)?)),
        SetActiveSlot::PACKET_ID => Ok(ServerBoundPacket::SetActiveSlot(decode_payload(payload)?)),
        DropItemStack::PACKET_ID => Ok(ServerBoundPacket::DropItemStack(decode_payload(payload)?)),
        MoveItemStack::PACKET_ID => Ok(ServerBoundPacket::MoveItemStack(decode_payload(payload)?)),
        _ => Err(CodecError::Other(format!("Unknown server bound packet id: {}", packet_id))),
    }
}
