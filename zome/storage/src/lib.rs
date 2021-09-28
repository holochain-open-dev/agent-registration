/**
 * Agent registration zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package agent registration zome
 * @author  pospi <pospi@spadgos.com>
 * @since   2020-03-22
 */
use hdk::prelude::*;
use hdk::hash_path::path::Component;

pub use hc_zome_agent_registration_storage_consts::*;

/// Determine path for an individual agent's registration in the DNA
pub fn path_for_agent(agent_address: &AgentPubKey) -> Path {
    Path::from(vec![AGENT_ROOT_ANCHOR_ID.into(), Component::from(agent_address.get_raw_39().to_vec())])
}

/// Load the well-known base anchor for agent queries.
pub fn get_root_anchor() -> Path {
    Path::from(vec![AGENT_ROOT_ANCHOR_ID.into()])
}
