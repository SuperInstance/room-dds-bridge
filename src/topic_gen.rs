//! Generate DDS topic definitions from room schemas.

use crate::{Room, TopicDef};

/// Generate a topic definition from a room and its schema.
pub fn generate_topic(room: &Room) -> TopicDef {
    let name = super::room_config::room_topic_name(room);
    let type_name = format!("{}Type", name);
    let key_fields: Vec<String> = room
        .schema
        .fields
        .iter()
        .filter(|f| f.is_key)
        .map(|f| f.name.clone())
        .collect();

    TopicDef {
        name,
        type_name,
        key_fields,
    }
}

/// Generate topic definitions for all rooms.
pub fn generate_topics(rooms: &[Room]) -> Vec<TopicDef> {
    rooms.iter().map(generate_topic).collect()
}

/// Validate that a topic definition is well-formed.
pub fn validate_topic(topic: &TopicDef) -> Result<(), String> {
    if topic.name.is_empty() {
        return Err("topic name must not be empty".into());
    }
    if topic.type_name.is_empty() {
        return Err("type name must not be empty".into());
    }
    Ok(())
}
