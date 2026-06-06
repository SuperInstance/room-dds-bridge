//! Build domain mapping from room topology.

use crate::{Domain, DomainBridge, DomainMap, Passage};

use std::collections::HashMap;

/// Build domain map: rooms connected by passages share a domain.
/// Disconnected components get separate domains.
pub fn build_domain_map(
    room_ids: &[String],
    passages: &[Passage],
) -> DomainMap {
    // Union-Find for connected components
    let mut parent: HashMap<String, String> = HashMap::new();
    for id in room_ids {
        parent.insert(id.clone(), id.clone());
    }

    fn find(parent: &mut HashMap<String, String>, x: &str) -> String {
        let root = if parent[x] == x {
            x.to_owned()
        } else {
            let p = parent[x].clone();
            let root = find(parent, &p);
            parent.insert(x.to_owned(), root.clone());
            root
        };
        root
    }

    fn union(parent: &mut HashMap<String, String>, a: &str, b: &str) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb {
            parent.insert(rb, ra);
        }
    }

    for p in passages {
        if parent.contains_key(&p.from) && parent.contains_key(&p.to) {
            union(&mut parent, &p.from, &p.to);
        }
    }

    // Group rooms by root
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for id in room_ids {
        let root = find(&mut parent, id);
        groups.entry(root).or_default().push(id.clone());
    }

    // Assign domain IDs
    let mut domains: Vec<Domain> = Vec::new();
    let mut sorted_groups: Vec<_> = groups.into_values().collect();
    // Deterministic ordering: sort rooms within each group, then sort groups by first room
    for g in &mut sorted_groups {
        g.sort();
    }
    sorted_groups.sort_by(|a, b| a[0].cmp(&b[0]));

    for (i, rooms) in sorted_groups.into_iter().enumerate() {
        domains.push(Domain {
            id: i as u32,
            rooms,
        });
    }

    // Build bridges: for each passage that crosses domains
    let mut domain_of: HashMap<String, u32> = HashMap::new();
    for d in &domains {
        for r in &d.rooms {
            domain_of.insert(r.clone(), d.id);
        }
    }

    let mut bridge_map: HashMap<(u32, u32), Vec<String>> = HashMap::new();
    for p in passages {
        let (Some(&from_d), Some(&to_d)) = (domain_of.get(&p.from), domain_of.get(&p.to)) else {
            continue;
        };
        if from_d != to_d {
            let key = (from_d.min(to_d), from_d.max(to_d));
            bridge_map.entry(key).or_default().push(p.from.clone());
            bridge_map.entry(key).or_default().push(p.to.clone());
        }
    }

    let bridges: Vec<DomainBridge> = bridge_map
        .into_iter()
        .map(|((from, to), mut rooms)| {
            rooms.sort();
            rooms.dedup();
            DomainBridge {
                from_domain: from,
                to_domain: to,
                rooms,
            }
        })
        .collect();

    DomainMap { domains, bridges }
}

/// Find which domain a room belongs to.
pub fn room_domain_id(map: &DomainMap, room_id: &str) -> Option<u32> {
    for d in &map.domains {
        if d.rooms.contains(&room_id.to_owned()) {
            return Some(d.id);
        }
    }
    None
}
