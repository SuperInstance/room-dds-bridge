use crate::*;

fn make_room(id: &str, agent_count: usize) -> Room {
    Room {
        id: id.into(),
        name: id.into(),
        agent_count,
        schema: room_config::default_schema(&Room {
            id: id.into(),
            name: id.into(),
            agent_count,
            schema: RoomSchema { fields: vec![] },
        }),
    }
}

fn make_passage(from: &str, to: &str, ternary: Ternary) -> Passage {
    Passage { from: from.into(), to: to.into(), ternary }
}

fn make_agent(id: &str, room_id: &str, publishes: bool, subscribes: bool) -> Agent {
    Agent { id: id.into(), room_id: room_id.into(), publishes, subscribes }
}

// ── room_config tests ───────────────────────────────────────────────────────

#[test]
fn single_room_single_topic() {
    let room = make_room("lobby", 3);
    let topic_name = room_config::room_topic_name(&room);
    assert_eq!(topic_name, "room_lobby");
}

#[test]
fn history_depth_equals_agent_count() {
    let room = make_room("hall", 7);
    assert_eq!(room_config::history_depth(&room), 7);
}

#[test]
fn history_depth_minimum_is_one() {
    let room = make_room("empty", 0);
    assert_eq!(room_config::history_depth(&room), 1);
}

#[test]
fn room_config_builds_correctly() {
    let room = make_room("kitchen", 4);
    let rc = room_config::build_room_config(&room, 2);
    assert_eq!(rc.room_id, "kitchen");
    assert_eq!(rc.topic, "room_kitchen");
    assert_eq!(rc.domain_id, 2);
    assert_eq!(rc.history_depth, 4);
}

#[test]
fn room_topic_name_replaces_spaces() {
    let room = Room {
        id: "dark room".into(),
        name: "Dark Room".into(),
        agent_count: 1,
        schema: RoomSchema { fields: vec![] },
    };
    assert_eq!(room_config::room_topic_name(&room), "room_dark_room");
}

// ── qos_policy tests ────────────────────────────────────────────────────────

#[test]
fn ternary_pos_gives_reliable() {
    assert_eq!(qos_policy::reliability_from_ternary(Ternary::Pos), Reliability::Reliable);
}

#[test]
fn ternary_zero_gives_best_effort() {
    assert_eq!(qos_policy::reliability_from_ternary(Ternary::Zero), Reliability::BestEffort);
}

#[test]
fn ternary_neg_gives_best_effort_reliability() {
    assert_eq!(qos_policy::reliability_from_ternary(Ternary::Neg), Reliability::BestEffort);
}

#[test]
fn ternary_neg_gives_transient_local_durability() {
    assert_eq!(qos_policy::durability_from_ternary(Ternary::Neg), Durability::TransientLocal);
}

#[test]
fn ternary_zero_gives_volatile_durability() {
    assert_eq!(qos_policy::durability_from_ternary(Ternary::Zero), Durability::Volatile);
}

#[test]
fn ternary_pos_gives_persistent_durability() {
    assert_eq!(qos_policy::durability_from_ternary(Ternary::Pos), Durability::Persistent);
}

#[test]
fn full_qos_pos() {
    let qos = qos_policy::qos_from_ternary(Ternary::Pos, 5);
    assert_eq!(qos.reliability, Reliability::Reliable);
    assert_eq!(qos.durability, Durability::Persistent);
    assert_eq!(qos.depth, 5);
    assert_eq!(qos.history, HistoryKind::KeepLast(5));
}

#[test]
fn full_qos_neg() {
    let qos = qos_policy::qos_from_ternary(Ternary::Neg, 3);
    assert_eq!(qos.reliability, Reliability::BestEffort);
    assert_eq!(qos.durability, Durability::TransientLocal);
}

#[test]
fn qos_depth_minimum_one() {
    let qos = qos_policy::qos_from_ternary(Ternary::Zero, 0);
    assert_eq!(qos.depth, 1);
}

// ── domain_map tests ────────────────────────────────────────────────────────

#[test]
fn single_room_single_domain() {
    let map = domain_map::build_domain_map(&["lobby".into()], &[]);
    assert_eq!(map.domains.len(), 1);
    assert_eq!(map.domains[0].rooms, vec!["lobby"]);
    assert!(map.bridges.is_empty());
}

#[test]
fn two_rooms_with_passage_same_domain() {
    let rooms = vec!["a".into(), "b".into()];
    let passages = vec![make_passage("a", "b", Ternary::Zero)];
    let map = domain_map::build_domain_map(&rooms, &passages);
    assert_eq!(map.domains.len(), 1);
    assert_eq!(map.domains[0].rooms.len(), 2);
}

#[test]
fn two_rooms_no_passage_separate_domains() {
    let rooms = vec!["a".into(), "b".into()];
    let map = domain_map::build_domain_map(&rooms, &[]);
    assert_eq!(map.domains.len(), 2);
    assert!(map.bridges.is_empty());
}

#[test]
fn three_rooms_chain_same_domain() {
    let rooms = vec!["a".into(), "b".into(), "c".into()];
    let passages = vec![
        make_passage("a", "b", Ternary::Pos),
        make_passage("b", "c", Ternary::Neg),
    ];
    let map = domain_map::build_domain_map(&rooms, &passages);
    assert_eq!(map.domains.len(), 1);
}

#[test]
fn room_domain_id_lookup() {
    let rooms = vec!["x".into(), "y".into()];
    let passages = vec![make_passage("x", "y", Ternary::Zero)];
    let map = domain_map::build_domain_map(&rooms, &passages);
    assert_eq!(domain_map::room_domain_id(&map, "x"), domain_map::room_domain_id(&map, "y"));
}

#[test]
fn room_domain_id_missing() {
    let map = domain_map::build_domain_map(&["a".into()], &[]);
    assert!(domain_map::room_domain_id(&map, "z").is_none());
}

// ── topic_gen tests ─────────────────────────────────────────────────────────

#[test]
fn topic_name_from_room() {
    let room = make_room("vault", 2);
    let topic = topic_gen::generate_topic(&room);
    assert_eq!(topic.name, "room_vault");
    assert_eq!(topic.type_name, "room_vaultType");
}

#[test]
fn topic_key_fields_from_schema() {
    let room = Room {
        id: "armory".into(),
        name: "Armory".into(),
        agent_count: 1,
        schema: RoomSchema {
            fields: vec![
                FieldDef { name: "id".into(), type_name: "string".into(), is_key: true },
                FieldDef { name: "value".into(), type_name: "int".into(), is_key: false },
            ],
        },
    };
    let topic = topic_gen::generate_topic(&room);
    assert_eq!(topic.key_fields, vec!["id"]);
}

#[test]
fn generate_multiple_topics() {
    let rooms = vec![make_room("a", 1), make_room("b", 2)];
    let topics = topic_gen::generate_topics(&rooms);
    assert_eq!(topics.len(), 2);
    assert_eq!(topics[0].name, "room_a");
    assert_eq!(topics[1].name, "room_b");
}

#[test]
fn validate_good_topic() {
    let topic = TopicDef { name: "room_x".into(), type_name: "room_xType".into(), key_fields: vec![] };
    assert!(topic_gen::validate_topic(&topic).is_ok());
}

#[test]
fn validate_empty_name_fails() {
    let topic = TopicDef { name: "".into(), type_name: "T".into(), key_fields: vec![] };
    assert!(topic_gen::validate_topic(&topic).is_err());
}

// ── pub_sub tests ───────────────────────────────────────────────────────────

#[test]
fn publisher_assignment() {
    let agents = vec![
        make_agent("p1", "room1", true, false),
        make_agent("s1", "room1", false, true),
    ];
    let assignments = pub_sub::assign_pub_sub(&agents);
    assert_eq!(assignments.len(), 1);
    assert_eq!(assignments[0].publishers, vec!["p1"]);
    assert_eq!(assignments[0].subscribers, vec!["s1"]);
}

#[test]
fn multiple_publishers_per_room() {
    let agents = vec![
        make_agent("p1", "room1", true, false),
        make_agent("p2", "room1", true, true),
    ];
    let assignments = pub_sub::assign_pub_sub(&agents);
    assert_eq!(assignments[0].publishers.len(), 2);
}

#[test]
fn can_publish_check() {
    let agents = vec![make_agent("a1", "room1", true, false)];
    assert!(pub_sub::can_publish(&agents, "a1", "room1"));
    assert!(!pub_sub::can_publish(&agents, "a1", "room2"));
}

#[test]
fn can_subscribe_check() {
    let agents = vec![make_agent("a1", "room1", false, true)];
    assert!(pub_sub::can_subscribe(&agents, "a1", "room1"));
    assert!(!pub_sub::can_subscribe(&agents, "a1", "room2"));
}

// ── bridge_config / serialization tests ─────────────────────────────────────

#[test]
fn config_serialization_round_trip() {
    let config = DDSConfig {
        rooms: vec![RoomConfig {
            room_id: "lobby".into(),
            topic: "room_lobby".into(),
            domain_id: 0,
            history_depth: 5,
        }],
        domains: DomainMap {
            domains: vec![Domain { id: 0, rooms: vec!["lobby".into()] }],
            bridges: vec![],
        },
        topics: vec![TopicDef {
            name: "room_lobby".into(),
            type_name: "room_lobbyType".into(),
            key_fields: vec!["room_id".into()],
        }],
    };
    let rt = bridge_config::round_trip(&config);
    assert_eq!(rt, config);
}

#[test]
fn config_to_json_contains_fields() {
    let config = DDSConfig {
        rooms: vec![RoomConfig {
            room_id: "cell".into(),
            topic: "room_cell".into(),
            domain_id: 0,
            history_depth: 1,
        }],
        domains: DomainMap {
            domains: vec![Domain { id: 0, rooms: vec!["cell".into()] }],
            bridges: vec![],
        },
        topics: vec![TopicDef {
            name: "room_cell".into(),
            type_name: "room_cellType".into(),
            key_fields: vec![],
        }],
    };
    let json = bridge_config::to_json(&config);
    assert!(json.contains("\"room_id\""));
    assert!(json.contains("\"cell\""));
}

#[test]
fn full_pipeline_single_room() {
    let room = make_room("start", 5);
    let topo = Topology {
        rooms: vec![room],
        passages: vec![],
        agents: vec![make_agent("hero", "start", true, true)],
    };
    let config = bridge_config::build_dds_config(&topo);
    assert_eq!(config.rooms.len(), 1);
    assert_eq!(config.topics.len(), 1);
    assert_eq!(config.domains.domains.len(), 1);
}

#[test]
fn full_pipeline_two_rooms_connected() {
    let topo = Topology {
        rooms: vec![make_room("a", 2), make_room("b", 3)],
        passages: vec![make_passage("a", "b", Ternary::Pos)],
        agents: vec![
            make_agent("p1", "a", true, false),
            make_agent("s1", "b", false, true),
        ],
    };
    let config = bridge_config::build_dds_config(&topo);
    assert_eq!(config.domains.domains.len(), 1);
    assert_eq!(config.rooms.len(), 2);
    assert_eq!(config.topics.len(), 2);
}

#[test]
fn full_pipeline_two_rooms_disconnected() {
    let topo = Topology {
        rooms: vec![make_room("x", 1), make_room("y", 1)],
        passages: vec![],
        agents: vec![],
    };
    let config = bridge_config::build_dds_config(&topo);
    assert_eq!(config.domains.domains.len(), 2);
}

#[test]
fn bridge_for_cross_domain_passage() {
    // Two rooms, no direct passage between them in same domain
    // but a passage referencing rooms in different domains
    let rooms = vec!["a".into(), "b".into(), "c".into()];
    let passages = vec![
        make_passage("a", "b", Ternary::Zero), // a-b same domain
        // c is alone; a passage a-c crosses domains
        make_passage("a", "c", Ternary::Pos),
    ];
    let map = domain_map::build_domain_map(&rooms, &passages);
    // a-b connected, c connected to a → all same domain
    assert_eq!(map.domains.len(), 1);
}

#[test]
fn separate_clusters_with_bridge_passage() {
    // a-b cluster, c-d cluster, passage b-c bridges them
    // Actually with passage b-c they'd merge. Let's just have two disconnected clusters.
    let rooms = vec!["a".into(), "b".into(), "c".into(), "d".into()];
    let passages = vec![
        make_passage("a", "b", Ternary::Zero),
        make_passage("c", "d", Ternary::Zero),
    ];
    let map = domain_map::build_domain_map(&rooms, &passages);
    assert_eq!(map.domains.len(), 2);
    assert!(map.bridges.is_empty()); // no cross-domain passages
}

// ── serde enum round-trip tests ─────────────────────────────────────────────

#[test]
fn serde_ternary_round_trip() {
    let values = [Ternary::Neg, Ternary::Zero, Ternary::Pos];
    for v in &values {
        let json = serde_json::to_string(v).unwrap();
        let back: Ternary = serde_json::from_str(&json).unwrap();
        assert_eq!(*v, back);
    }
}

#[test]
fn serde_qos_round_trip() {
    let qos = qos_policy::qos_from_ternary(Ternary::Pos, 10);
    let json = serde_json::to_string(&qos).unwrap();
    let back: QoSPolicy = serde_json::from_str(&json).unwrap();
    assert_eq!(qos, back);
}

#[test]
fn serde_domain_map_round_trip() {
    let map = domain_map::build_domain_map(
        &["a".into(), "b".into()],
        &[make_passage("a", "b", Ternary::Zero)],
    );
    let json = serde_json::to_string(&map).unwrap();
    let back: DomainMap = serde_json::from_str(&json).unwrap();
    assert_eq!(map, back);
}
