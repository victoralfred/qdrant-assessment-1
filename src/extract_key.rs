use qdrant_client::qdrant::{shard_key::Key, ShardKey};

pub fn extract_keyword(
    shard_key: Option<ShardKey>
) -> String {

    match shard_key {
        Some(ShardKey { key: Some(Key::Keyword(ref k)) }) => k.clone(), // Clone the string
        _ => String::from(""), // Return a default string for None case
    }
}