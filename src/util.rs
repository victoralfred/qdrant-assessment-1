use std::cmp::Ordering;
use qdrant_client::qdrant::SearchResponse;

#[allow(dead_code)]
fn sort_score_in_ascending_order(mut response: SearchResponse) -> SearchResponse {
    // Sort in-place by score in ascending order (lowest score first)
    response.result.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal));
    response  // Return the sorted response
}
#[allow(dead_code)]
fn sort_score_in_descending_order(mut response: SearchResponse) -> SearchResponse {
    // Sort in-place by score in descending order
    response.result.sort_by(|a,b|
         b.score.partial_cmp(&a.score)
         .unwrap_or(Ordering::Equal));
    response  // Return the sorted response
}
