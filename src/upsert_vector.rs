
use qdrant_client::{qdrant::{shard_key::{self}, PointStruct, ShardKey, UpsertPointsBuilder},Payload, Qdrant, QdrantError};

use serde_json::json;

use crate::extract_key::extract_keyword;



pub async fn upsert_vector(
    collection_name: &str,
    named_shard_key: String
)
-> Result<(), QdrantError>{
    let client = Qdrant::from_url("http://localhost:6334").build()?;
    // Create a new ShardKey
    let shard_keys: ShardKey = ShardKey {
        key: Some(shard_key::Key::Keyword(named_shard_key)),
    };
        match  client
        .upsert_points(
            UpsertPointsBuilder::new(
                collection_name,
                vec![
                    PointStruct::new(
                        1,
                        vec![0.9, 0.1, 0.1],
                        Payload::try_from(json!(
                            {"key": "Find items that match the meaning or intent behind a query, using natural language processing"}
                        ))
                        .unwrap(),
                    ),
                    PointStruct::new(
                        2,
                        vec![0.1, 0.9, 0.1],
                        Payload::try_from(json!(
                            {"key": "Natural language queries, question answering"}
                        ))
                        .unwrap(),
                    ),
                    PointStruct::new(
                        3,
                        vec![0.1, 0.1, 0.9],
                        Payload::try_from(json!(
                            {"key": "General search queries, finding specific information"}
                        ))
                        .unwrap(),
                    ),
                ],
        
            )
            .shard_key_selector(extract_keyword(Some(shard_keys)))
        )
        .await{
            Ok(msg) => {
                msg 
            },  // Do nothing if successful
            Err(e) => {
                return Err(e);
            }
    };
        Ok(())
}
