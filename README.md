# room-dds-bridge

Auto-configure DDS (Data Distribution Service) domains from [ternary-mud](https://en.wikipedia.org/wiki/MUD) room topology.

Each room becomes a DDS topic, passages become domain bridges, and the ternary **{-1, 0, +1}** state drives QoS policies.

## Overview

**ternary-mud** is a MUD engine where rooms and passages carry a ternary state (`Neg`, `Zero`, `Pos`). **room-dds-bridge** translates that topology into a complete DDS middleware configuration — no manual DDS setup required.

```
Room Topology          →    DDS Configuration
─────────────────         ──────────────────────
Room "lobby"          →    Topic "room_lobby" on Domain 0
Passage (lobby, hall) →    Same domain (bridged internally)
Ternary +1            →    RELIABLE + PERSISTENT QoS
Ternary  0            →    BEST_EFFORT + VOLATILE QoS
Ternary -1            →    BEST_EFFORT + TRANSIENT_LOCAL QoS
Agent count = 5       →    HistoryDepth = 5
```

## Architecture

| Module | Purpose |
|---|---|
| `room_config` | Map rooms → DDS topics, compute history depth from agent count |
| `qos_policy` | Derive QoS policies (reliability, durability, history) from ternary state |
| `domain_map` | Union-Find connected components → domain assignment, cross-domain bridges |
| `topic_gen` | Generate DDS topic definitions (name, type, key fields) from room schemas |
| `pub_sub` | Assign publishers/subscribers per room from agent declarations |
| `bridge_config` | Export full `DDSConfig` as serializable JSON |

## Core Types

```rust
struct DDSConfig {
    rooms: Vec<RoomConfig>,     // Per-room DDS settings
    domains: DomainMap,          // Domain partitioning + bridges
    topics: Vec<TopicDef>,       // Generated topic definitions
}

struct QoSPolicy {
    reliability: Reliability,    // Reliable | BestEffort
    durability: Durability,      // Volatile | TransientLocal | Persistent
    history: HistoryKind,        // KeepLast(n) | KeepAll
    depth: usize,
}
```

### Ternary → QoS Mapping

| Ternary | Reliability | Durability |
|---------|------------|------------|
| `Pos (+1)` | `Reliable` | `Persistent` |
| `Zero (0)` | `BestEffort` | `Volatile` |
| `Neg (-1)` | `BestEffort` | `TransientLocal` |

### Domain Rules

- Rooms connected by passages share a domain (union-find)
- Disconnected room clusters get separate domains
- Passages crossing domain boundaries produce `DomainBridge` entries

## Usage

```rust
use room_dds_bridge::*;

let topology = Topology {
    rooms: vec![
        Room {
            id: "lobby".into(),
            name: "Grand Lobby".into(),
            agent_count: 5,
            schema: RoomSchema {
                fields: vec![
                    FieldDef { name: "room_id".into(), type_name: "string".into(), is_key: true },
                    FieldDef { name: "name".into(), type_name: "string".into(), is_key: false },
                ],
            },
        },
    ],
    passages: vec![],
    agents: vec![Agent {
        id: "bot1".into(),
        room_id: "lobby".into(),
        publishes: true,
        subscribes: true,
    }],
};

let config = bridge_config::build_dds_config(&topology);
let json = bridge_config::to_json(&config);
println!("{}", json);
```

## Testing

```bash
cargo test
```

30+ tests covering:
- Single/multi-room domain assignment
- Connected vs disconnected topology
- All ternary → QoS mappings
- History depth from agent count
- Topic generation and validation
- Publisher/subscriber assignment
- JSON serialization round-trips
- Full pipeline (topology → config → JSON → config)

## License

MIT
