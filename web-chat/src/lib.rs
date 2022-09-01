use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

// p569
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ClientPacket {             // was:FromClient
    Join {
        group: Arc<String>,         // was:group_name
    },
    Post {
        group: Arc<String>,
        message: Arc<String>,
    },
}

pub enum ServerPacket {             // was:FromServer, enum
    Message {                       // struct variant
        group: Arc<String>,         // Arc allows server to reuse strings for messages and group names
        message: Arc<String>,       // These strings are not reused for serialization/deserialization
    },
    Error(String),                  // tuple variant
}

#[test]
fn test_client_packet_json_is_correct() {

    let target = ClientPacket::Post {
        group: Arc::new("Cats".to_string()),
        message: Arc::new("Hello cats!".to_string()),
    };

    let serialized = serde_json::to_string(&target).unwrap();
    let deserialized = serde_json::from_str::<ClientPacket>(&serialized).unwrap();

    assert_eq!(serialized, "{\"Post\":{\"group_name\":\"Cats\",\"message\":\"Hello cats!\"}}");
    assert_eq!(serialized, r#"{"Post":{"group_name":"Cats","message":"Hello cats!"}}"#); // raw string p74
    assert_eq!(deserialized, target);
}