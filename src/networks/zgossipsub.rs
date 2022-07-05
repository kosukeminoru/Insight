use libp2p::gossipsub;
use libp2p::gossipsub::{GossipsubMessage, MessageAuthenticity, MessageId, ValidationMode};
use libp2p::identity::Keypair;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::time::Duration;

pub fn create_gossip(local_key: Keypair) -> gossipsub::Gossipsub {
    let message_id_fn = |message: &GossipsubMessage| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        MessageId::from(s.finish().to_string())
    };
    // Set a custom gossipsub
    let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(1)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the
        // same content will be propagated.
        .build()
        .expect("Valid config");
    // build a gossipsub network behaviour
    /* dev metrics */
    let mut gossipsub: gossipsub::Gossipsub =
        gossipsub::Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
            .expect("Correct configuration");
    gossipsub
        .with_peer_score(
            gossipsub::PeerScoreParams {
                topics: HashMap::new(),
                topic_score_cap: 3600.0,
                app_specific_weight: 30.0,
                ip_colocation_factor_weight: -10.0,
                ip_colocation_factor_threshold: 10.0,
                ip_colocation_factor_whitelist: HashSet::new(),
                behaviour_penalty_weight: -10.0,
                behaviour_penalty_threshold: 0.0,
                behaviour_penalty_decay: 0.2,
                decay_interval: Duration::from_secs(1),
                decay_to_zero: 0.1,
                retain_score: Duration::from_secs(3600),
            },
            gossipsub::PeerScoreThresholds {
                gossip_threshold: -10.0,
                publish_threshold: -50.0,
                graylist_threshold: -80.0,
                accept_px_threshold: 60.0,
                opportunistic_graft_threshold: 120.0,
            },
        )
        .expect("Correct peer score");

    // subscribes to our topic
    gossipsub
}
