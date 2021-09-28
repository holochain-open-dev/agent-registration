use serde::*;
use holo_hash::{AgentPubKey};

/// WASM API query parameters for determining if an agent is registered
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationQueryParams {
    pub pub_key: AgentPubKey,
}
