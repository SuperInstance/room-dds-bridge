//! # room-dds-bridge
//!
//! Auto-configure DDS (Data Distribution Service) domains from ternary-mud room topology.
//!
//! Each room becomes a DDS topic, passages become domain bridges, and the ternary
//! {-1, 0, +1} state drives QoS policies.

pub mod bridge_config;
pub mod domain_map;
pub mod pub_sub;
pub mod qos_policy;
pub mod room_config;
pub mod topic_gen;

use serde::{Deserialize, Serialize};

// ── Core enums ──────────────────────────────────────────────────────────────

/// DDS reliability policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reliability {
    Reliable,
    BestEffort,
}

/// DDS durability policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Durability {
    Volatile,
    TransientLocal,
    Persistent,
}

/// DDS history kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryKind {
    KeepLast(usize),
    KeepAll,
}

/// Ternary state from the mud engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ternary {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

// ── Core structs ────────────────────────────────────────────────────────────

/// Full QoS policy derived from ternary state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QoSPolicy {
    pub reliability: Reliability,
    pub durability: Durability,
    pub history: HistoryKind,
    pub depth: usize,
}

/// Per-room DDS configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomConfig {
    pub room_id: String,
    pub topic: String,
    pub domain_id: u32,
    pub history_depth: usize,
}

/// A single DDS domain containing one or more rooms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Domain {
    pub id: u32,
    pub rooms: Vec<String>,
}

/// A bridge connecting two domains for specific rooms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainBridge {
    pub from_domain: u32,
    pub to_domain: u32,
    pub rooms: Vec<String>,
}

/// Full domain mapping: domains and inter-domain bridges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainMap {
    pub domains: Vec<Domain>,
    pub bridges: Vec<DomainBridge>,
}

/// A DDS topic definition generated from a room schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicDef {
    pub name: String,
    pub type_name: String,
    pub key_fields: Vec<String>,
}

/// Complete DDS configuration, serializable to JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DDSConfig {
    pub rooms: Vec<RoomConfig>,
    pub domains: DomainMap,
    pub topics: Vec<TopicDef>,
}

/// A room in the mud topology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub agent_count: usize,
    pub schema: RoomSchema,
}

/// Room data schema for topic generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomSchema {
    pub fields: Vec<FieldDef>,
}

/// A field in a room schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub type_name: String,
    pub is_key: bool,
}

/// A passage connecting two rooms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Passage {
    pub from: String,
    pub to: String,
    pub ternary: Ternary,
}

/// An agent in the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub room_id: String,
    pub publishes: bool,
    pub subscribes: bool,
}

/// Publisher/subscriber assignment for a room.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PubSubAssignment {
    pub room_id: String,
    pub publishers: Vec<String>,
    pub subscribers: Vec<String>,
}

/// Full topology input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topology {
    pub rooms: Vec<Room>,
    pub passages: Vec<Passage>,
    pub agents: Vec<Agent>,
}

#[cfg(test)]
mod tests;
