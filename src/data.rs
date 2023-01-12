use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Permission {
    id: String,
    object: String,
    created: String,
    allow_create_engine: bool,
    allow_sampling: bool,
    allow_logprobs: bool,
    allow_search_indices: bool,
    allow_view: bool,
    allow_fine_tuning: bool,
    organization: String,
    group: String,
    is_blocking: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    id: String,
    object: String,
    created: DateTime<Utc>,
    owned_by: String,
    permission: Vec<Permission>,
    root: String,
    parent: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelResponse {
    object: String,
    data: Vec<Model>,
}
