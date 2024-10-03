## Sorting Order:

1.    Sorting Order: Is there a way to reverse the order of query results? We haven’t been able to find an asc/desc order parameter in the scroll, search, discover, or query APIs. Could you clarify if this functionality is supported?
2.    <i><ins>Sorting by Payload Key: We would like to sort query results by custom fields like date (stored in the payload). While we understand sorting by payload keys is possible (e.g., query=models.OrderByQuery(order_by="date")), is it possible to add a secondary condition to sort by score? Specifically, we want to prefetch sorted by date and then refine the results sorted by score.</ins></i>
-   Potential Feature request


    [Discord channel link](https://discord.com/channels/907569970500743200/1193336069987516476/1286871212601114758)

    [Examples](util.rs)

    Given the SearchResponse from using the [Similarity Search API](https://qdrant.tech/documentation/concepts/search/#search-api) e.g;

    ```rust
    pub async fn query_point(
            client: &Qdrant,
            collection_name: &str
            )-> Result<SearchResponse, QdrantError>{
            
            match client
                .search_points(
                    SearchPointsBuilder::new(collection_name,[0.1, 0.9, 0.1],3)
                    .with_payload(true)
                    .params(SearchParamsBuilder::default().exact(true))
                )
                .await{
                    Ok(result) =>{
                    return Ok(result)
                    },
                    Err(e) =>{
                        return Err(e);
                    }
                };

        }
    ````
    Example response;

    ```javascript
        SearchResponse {
            result: [ScoredPoint {
                id: Some(PointId {
                    point_id_options: Some(Num(2))
                }),
                payload: {
                    "key": Value {
                        kind: Some(StringValue("Natural language queries, question answering"))
                    }
                },
                score: 1.0000001,
                version: 0,
                vectors: None,
                shard_key: Some(ShardKey {
                    key: Some(Keyword("netherland"))
                }),
                order_value: None
            }, ScoredPoint {
                id: Some(PointId {
                    point_id_options: Some(Num(1))
                }),
                payload: {
                    "key": Value {
                        kind: Some(StringValue("Find items that match the meaning or intent behind a query, using natural language processing"))
                    }
                },
                score: 0.2289157,
                version: 0,
                vectors: None,
                shard_key: Some(ShardKey {
                    key: Some(Keyword("netherland"))
                }),
                order_value: None
            }, ScoredPoint {
                id: Some(PointId {
                    point_id_options: Some(Num(3))
                }),
                payload: {
                    "key": Value {
                        kind: Some(StringValue("General search queries, finding specific information"))
                    }
                },
                score: 0.22891569,
                version: 0,
                vectors: None,
                shard_key: Some(ShardKey {
                    key: Some(Keyword("netherland"))
                }),
                order_value: None
            }],
            time: 0.006925261
        }
    ```

    You can write a custom function to order the score in ASC/DESC order

    ```rust
    fn sort_score_in_ascending_order(
        mut response: SearchResponse
        ) -> SearchResponse {
                
        // Sort in-place by score in ascending order (lowest score first)
        response.result.sort_by(|a, b| a.score.partial_cmp(&b.score)
        .unwrap_or(Ordering::Equal));

        response
    }

    fn sort_score_in_descending_order(
        mut response: SearchResponse
        ) -> SearchResponse {

        // Sort in-place by score in descending order
        response.result.sort_by(|a,b|b.score.partial_cmp(&a.score)
        .unwrap_or(Ordering::Equal));

        response
    }
    ``` 	

3.  Is there a way to retrieve the similarity score between two specific points?
    
    You can do this by writing custom scripts. This is an example using a custom Python script, e.g;

- If you have two [Points](https://qdrant.tech/documentation/concepts/points/#point-ids);

        ```bash
        vector_1=[{
            'id': 1,
            'payload': {
                    'key': 'Find items that match the meaning or intent behind a query, using natural language processing'
            },
            'vector': [0.9878784,
            0.10976427,
            0.10976427],
            'shard_key': 'netherland'
            }]

        vector_2 =[{
            'id': 2,
            'payload': {
                    'key': 'Natural language queries, question answering'
            },
            'vector': [0.10976427,
                0.9878784,
                0.10976427],
            'shard_key': 'netherland'
            }]
        ```
You can compare the similarity score using a simple numpy dot product and norms of the vectors, e.g;

```python
        import numpy as np

        def cosine_similarity(vector1, vector2):
            """computes the cosine similarity between two vectors using the dot product and norms of the vectors"""
            dot_product = np.dot(vector1, vector2)
            norm_a = np.linalg.norm(vector1)
            norm_b = np.linalg.norm(vector2)
            return dot_product / (norm_a * norm_b)

        score = cosine_similarity(
            np.array(vector_1[0]['vector']),
            np.array(vector_2[0]['vector'])
        )
        print(f'{score}')
    ```
