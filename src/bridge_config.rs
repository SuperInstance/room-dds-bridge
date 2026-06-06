//! Export full DDS configuration as serializable config.

use crate::{
    domain_map, room_config, topic_gen, DDSConfig, RoomConfig, Topology,
};

/// Build a complete DDS configuration from a topology.
pub fn build_dds_config(topology: &Topology) -> DDSConfig {
    let room_ids: Vec<String> = topology.rooms.iter().map(|r| r.id.clone()).collect();
    let domain_map = domain_map::build_domain_map(&room_ids, &topology.passages);

    let rooms: Vec<RoomConfig> = topology
        .rooms
        .iter()
        .map(|r| {
            let domain_id = domain_map::room_domain_id(&domain_map, &r.id).unwrap_or(0);
            room_config::build_room_config(r, domain_id)
        })
        .collect();

    let topics = topic_gen::generate_topics(&topology.rooms);

    DDSConfig {
        rooms,
        domains: domain_map,
        topics,
    }
}

/// Serialize a DDSConfig to a JSON string.
pub fn to_json(config: &DDSConfig) -> String {
    serde_json::to_string_pretty(config).expect("serialization should not fail")
}

/// Deserialize a DDSConfig from a JSON string.
pub fn from_json(json: &str) -> Result<DDSConfig, serde_json::Error> {
    serde_json::from_str(json)
}

/// Round-trip: serialize then deserialize.
pub fn round_trip(config: &DDSConfig) -> DDSConfig {
    let json = to_json(config);
    from_json(&json).expect("round-trip should not fail")
}
