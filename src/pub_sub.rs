//! Publisher/subscriber assignment.

use crate::{Agent, PubSubAssignment};

/// Assign publishers and subscribers per room based on agent declarations.
pub fn assign_pub_sub(agents: &[Agent]) -> Vec<PubSubAssignment> {
    let mut map: std::collections::HashMap<String, PubSubAssignment> =
        std::collections::HashMap::new();

    for a in agents {
        let entry = map.entry(a.room_id.clone()).or_insert_with(|| PubSubAssignment {
            room_id: a.room_id.clone(),
            publishers: Vec::new(),
            subscribers: Vec::new(),
        });
        if a.publishes {
            entry.publishers.push(a.id.clone());
        }
        if a.subscribes {
            entry.subscribers.push(a.id.clone());
        }
    }

    let mut assignments: Vec<_> = map.into_values().collect();
    assignments.sort_by(|a, b| a.room_id.cmp(&b.room_id));
    assignments
}

/// Check if an agent can publish to a room.
pub fn can_publish(agents: &[Agent], agent_id: &str, room_id: &str) -> bool {
    agents
        .iter()
        .any(|a| a.id == agent_id && a.room_id == room_id && a.publishes)
}

/// Check if an agent can subscribe to a room.
pub fn can_subscribe(agents: &[Agent], agent_id: &str, room_id: &str) -> bool {
    agents
        .iter()
        .any(|a| a.id == agent_id && a.room_id == room_id && a.subscribes)
}
