//! Derive QoS policies from ternary state.

use crate::{Durability, HistoryKind, QoSPolicy, Reliability, Ternary};

/// Derive reliability from ternary state.
pub fn reliability_from_ternary(t: Ternary) -> Reliability {
    match t {
        Ternary::Pos => Reliability::Reliable,
        Ternary::Zero | Ternary::Neg => Reliability::BestEffort,
    }
}

/// Derive durability from ternary state.
pub fn durability_from_ternary(t: Ternary) -> Durability {
    match t {
        Ternary::Neg => Durability::TransientLocal,
        Ternary::Zero => Durability::Volatile,
        Ternary::Pos => Durability::Persistent,
    }
}

/// Build a complete QoS policy from ternary state and depth.
pub fn qos_from_ternary(t: Ternary, depth: usize) -> QoSPolicy {
    let depth = depth.max(1);
    QoSPolicy {
        reliability: reliability_from_ternary(t),
        durability: durability_from_ternary(t),
        history: HistoryKind::KeepLast(depth),
        depth,
    }
}

/// Convenience: default QoS (BestEffort, Volatile).
pub fn default_qos(depth: usize) -> QoSPolicy {
    qos_from_ternary(Ternary::Zero, depth)
}
