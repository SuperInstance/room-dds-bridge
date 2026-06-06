//! Map room topology to DDS configurations.

use crate::{Room, RoomConfig, RoomSchema};

/// Generate a DDS topic name from a room.
pub fn room_topic_name(room: &Room) -> String {
    format!("room_{}", room.id.replace(' ', "_"))
}

/// Compute history depth from the agent count.
pub fn history_depth(room: &Room) -> usize {
    room.agent_count.max(1)
}

/// Build a RoomConfig from a Room given its assigned domain_id.
pub fn build_room_config(room: &Room, domain_id: u32) -> RoomConfig {
    RoomConfig {
        room_id: room.id.clone(),
        topic: room_topic_name(room),
        domain_id,
        history_depth: history_depth(room),
    }
}

/// Infer a RoomSchema from a room if none provided, returning a default.
pub fn default_schema(_room: &Room) -> RoomSchema {
    RoomSchema {
        fields: vec![
            crate::FieldDef {
                name: "room_id".into(),
                type_name: "string".into(),
                is_key: true,
            },
            crate::FieldDef {
                name: "name".into(),
                type_name: "string".into(),
                is_key: false,
            },
            crate::FieldDef {
                name: "agent_count".into(),
                type_name: "uint32".into(),
                is_key: false,
            },
        ],
    }
}
