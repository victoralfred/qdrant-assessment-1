use qdrant_client::{qdrant::{shard_key::Key, CreateShardKeyBuilder, CreateShardKeyRequestBuilder},Qdrant, QdrantError};


pub async fn create_shard_keys(
    collection_name: &str, 
    shard_key: Vec<&str>,
    client: &Qdrant
    )
    -> Result<(), QdrantError>{

    for &key in &shard_key{
        let collection_with_shard_key_request= CreateShardKeyRequestBuilder
        ::new(collection_name)
        .request(CreateShardKeyBuilder::default()
        .shard_key(Key::Keyword(key.to_string())));

            // captures the result of each shard key creation operation with a match
            match client.create_shard_key(collection_with_shard_key_request).await{
                Ok(_) => (),  // Do nothing if successful
                Err(e) => {
                        // Log or propagate the error, rather than ignoring it
                        eprintln!("Failed to create shard key for '{}': {:?}", key, e);
                        return Err(e)
                }
            };
    }
   Ok(())
}