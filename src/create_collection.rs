
use qdrant_client::{qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder},Qdrant, QdrantError};

pub async fn create_collection(
    collection_name: &str,
    client: &Qdrant
)-> Result<(), QdrantError>{

        // Create a collection with vectors of dimension 100
    match client.create_collection(
        CreateCollectionBuilder::new(collection_name)
        .vectors_config(VectorParamsBuilder::new(3, Distance::Cosine))
        .shard_number(4u32)
        .sharding_method(1),
    ).await{
        Ok(msg) => {
            println!("Colection Created Successfully {:?}", msg)
             
        },  // Do nothing if successful
            Err(e) => {
                // Log or propagate the error, rather than ignoring it
                eprintln!("Failed to create collection with error': {:?}", e);
            }
    };
    
    Ok(())
}