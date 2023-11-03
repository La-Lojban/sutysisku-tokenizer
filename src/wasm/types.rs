use serde::{Deserialize, Serialize};
use tsify::Tsify;

pub type TopKMatches = usize;
pub type Query = Vec<f32>;

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EmbeddedResource {
    pub title: String,
    pub embeddings: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Resource {
    pub embeddings: Vec<EmbeddedResource>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SearchResult {
    pub neighbors: Vec<String>,
}
