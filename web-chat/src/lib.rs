use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

// p569
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

pub enum FromServer {               // enum
    Message {                       // struct variant
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),                  // tuple variant
}

#[test]
fn test_from_clinet_json_is_correct() {

    let target = FromClient::Post {
        group_name: Arc::new("Cats".to_string()),
        message: Arc::new("Hello cats!".to_string()),
    };

    let serialized = serde_json::to_string(&target).unwrap();
    let deserialized = serde_json::from_str::<FromClient>(&serialized).unwrap();

    assert_eq!(serialized, "{\"Post\":{\"group_name\":\"Cats\",\"message\":\"Hello cats!\"}}");
    assert_eq!(serialized, r#"{"Post":{"group_name":"Cats","message":"Hello cats!"}}"#); // raw string p74
    assert_eq!(deserialized, target);
}