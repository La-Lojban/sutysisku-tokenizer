use crate::utils::set_panic_hook;
use crate::{hash, SearchResult};
use crate::{Query, Resource, SearchResults, TopKMatches};
use kiddo::float::distance::manhattan;
use kiddo::float::kdtree::KdTree;
// use kiddo::immutable::float::kdtree::ImmutableKdTree;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryInto};

use wasm_bindgen::prelude::*;

// Wasm has a 4GB memory limit. Should make sure the bucket size and capacity
// doesn't exceed it and cause stack overflow.
// More detail: https://v8.dev/blog/4gb-wasm-memory
const BUCKET_SIZE: usize = 32;

// pub type Tree = ImmutableKdTree<f32, u64, 384, 32>;
pub type Tree = KdTree<f32, u64, 384, BUCKET_SIZE, u16>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    // "IDX" is set to u16 to optimize CPU cache.
    // Read more: https://github.com/sdd/kiddo/blob/7a0bb6ecce39963b27ffdca913c6be7a265e3523/src/types.rs#L35
    pub tree: Tree,
    pub data: HashMap<u64, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QueryX {
    // TODO: support query in string
    // Phrase(String)
    Embeddings(Vec<f32>),
}

#[wasm_bindgen]
pub struct Sisku {
    index: Index,
}

#[wasm_bindgen]
impl Sisku {
    #[wasm_bindgen(constructor)]
    pub fn new(resource: Option<Resource>) -> Sisku {
        set_panic_hook();

        let resource: Resource = match resource {
            Some(res) => res,
            _ => Resource { embeddings: vec![] },
        };
        let data_vec: Vec<(u64, String)> = resource
            .embeddings
            .iter()
            .map(|resource| resource.title.to_owned())
            .map(|document| (hash(&document), document))
            .collect();

        let data: HashMap<u64, String> = data_vec.clone().into_iter().collect();

        // let embeddings: Vec<[f32; 384]> = resource
        //     .embeddings
        //     .iter()
        //     .map(|embedding| {
        //         let mut e = embedding.embeddings.clone();
        //         e.resize(384, 0.0);
        //         e.try_into().unwrap()
        //     })
        //     .collect();

        // let tree: ImmutableKdTree<f32, u64, 384, 32> = ImmutableKdTree::new_from_slice(&embeddings);

        let mut tree: Tree = KdTree::new();
        resource
            .embeddings
            .iter()
            .zip(data_vec.iter())
            .for_each(|(resource, data)| {
                let mut embeddings = resource.embeddings.clone();
                embeddings.resize(384, 0.0);

                let query: &[f32; 384] = &embeddings.try_into().unwrap();
                // "item" holds the position of the document in "data"
                tree.add(query, data.0);
            });
        let index = Index { tree, data };

        Sisku { index }
    }

    #[wasm_bindgen]
    pub fn search(&self, query: Query, k: TopKMatches) -> SearchResults {
        let query: QueryX = QueryX::Embeddings(query);
        let mut query: Vec<f32> = match query {
            QueryX::Embeddings(q) => q.to_owned(),
        };
        query.resize(384, 0.0);

        let query: &[f32; 384] = &query.try_into().unwrap();
        let neighbors = self.index.tree.nearest_n(query, k, &manhattan);

        let mut result: Vec<SearchResult> = vec![];

        for neighbor in &neighbors {
            let doc = &self.index.data.get(&neighbor.item);
            if let Some(document) = doc {
                result.push(SearchResult {
                    item: document.to_string(),
                    distance: neighbor.distance,
                });
            }
        }

        let neighbors: Vec<SearchResult> = result.into_iter().collect();

        SearchResults { neighbors }
    }

    #[wasm_bindgen]
    pub fn add(&mut self, resource: Resource) {
        for item in &resource.embeddings {
            let mut embeddings = item.embeddings.clone();
            embeddings.resize(384, 0.0);

            let query: &[f32; 384] = &embeddings.try_into().unwrap();
            let doc = item.title.to_owned();
            let id = hash(&doc);
            self.index.data.insert(id, doc);
            self.index.tree.add(query, id);
        }
    }
}
