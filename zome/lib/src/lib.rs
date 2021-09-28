/**
 * Agent registration zome library API
 *
 * Contains helper methods that can be used to query agent registration data
 * from within either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package agent registration zome
 * @author  pospi <pospi@spadgos.com>
 * @since   2020-03-22
 */
use hdk::prelude::*;
use hdk::api::{
    get_links,
    entry_address,
};

use hc_zome_agent_registration_storage_consts::*;

/// Returns true if the given agent ID is a member of the local DNA
pub fn is_registered_agent(address: &Address) -> ZomeApiResult<bool> {
    let links_result = get_links(
        &get_root_entry_address()?,
        LinkMatch::Exactly(AGENT_ANCHOR_LINK_TYPE.into()),
        LinkMatch::Exactly(address.to_string().as_ref())
    )?.addresses();
    Ok(links_result.len() > 0)
}

/// Returns the addresses of all agents who have accessed the local DNA
pub fn get_registered_agents() -> ZomeApiResult<Vec<Address>> {
    Ok(get_links(&get_root_entry_address()?, LinkMatch::Exactly(AGENT_ANCHOR_LINK_TYPE.into()), LinkMatch::Any)?.addresses())
}

/// Load the well-known base anchor for agent queries.
/// You probably don't need to access this method directly, see other methods for top-level functionality.
pub fn get_root_entry() -> Entry {
    Entry::App(AGENT_ROOT_ANCHOR_ID.into(), AGENT_ROOT_ANCHOR_ID.into())
}

// Get the address of the well-known base anchor for agent queries.
pub fn get_root_entry_address() -> ZomeApiResult<Address> {
    entry_address(&get_root_entry())
}
