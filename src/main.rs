mod util;
mod create_collection;
mod create_shard_keys;
mod upsert_vector;
mod extract_key;
use extract_key::extract_keyword;
use prettytable::{row,Table};
use qdrant_client::{ Qdrant, QdrantError};
use create_collection::create_collection;
use create_shard_keys::create_shard_keys;
use upsert_vector::upsert_vector;


#[tokio::main]
async fn main() -> Result<(),QdrantError> {

    let collection_name = "research_papers_collection"; // Collection name
    let shard_keys = vec!["netherland","germany","france"]; //Vector of shard_keys
    let client = Qdrant::from_url("http://localhost:6334")
    .build()
        .map_err(|e| {
            eprintln!("Error creating Qdrant client: {:?}", e);
            e
        })?;

    // Create a collection
    create_collection(collection_name, &client).await?;

    // Create Shard key
    create_shard_keys(collection_name, shard_keys,&client).await?;
    // Get the shards of a Qdrant instance
    let collection_cluster_list = client.collection_cluster_info(collection_name).await?;

    // Create a table to display the cluster info
    let mut table = Table::new();
    table.add_row(row!["Type", "Shard ID", "Peer ID", "State", "Shard Key"]);

    // Add remote shard rows
    for remote_shard in collection_cluster_list.remote_shards{
        table.add_row(row![
            "Remote",
            remote_shard.shard_id,
            remote_shard.peer_id,
            format!("{:?}", remote_shard.state()),
            format!("{:?}", extract_keyword(remote_shard.shard_key))
        ]);
    }
    // Add local shard rows
    for local_shard in collection_cluster_list.local_shards{
        table.add_row(row![
            "Local",
            local_shard.shard_id,
            collection_cluster_list.peer_id,
            format!("{:?}", local_shard.state()),
            format!("{:?}", extract_keyword(local_shard.shard_key))
        ]);
    }
    // Print the table with cluster information
    table.printstd();
        // Upsert vector for a specific shard key
    if let Err(e) = upsert_vector(collection_name,String::from("netherland") ).await {
        eprintln!("Failed to upsert vector for shard 'netherland': {:?}", e);
            return Err(e);
    }
    Ok(())
}