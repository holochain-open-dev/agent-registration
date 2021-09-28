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
use hdk::api::{
    commit_entry,
    link_entries,
    AGENT_ADDRESS,
};

use hc_zome_agent_registration_storage_consts::*;
use hc_zome_agent_registration_lib::{get_root_entry, is_registered_agent};

/// Initialises data needed to query registered agents from the network.
/// Must be called at zome init time.
pub fn init() -> Result<(), String> {
    // ensure root entry first
    let root_address = commit_entry(&get_root_entry())?;    // :TODO: is this linearly adding provenances to the hashchain?

    let agent_address = Address::from(AGENT_ADDRESS.to_string());

    // Avoid duplicate linking if already registered
    if Ok(true) == is_registered_agent(&agent_address) {
        return Ok(());
    }

    // Not already registered- write link
    link_entries(&root_address, &agent_address, AGENT_ANCHOR_LINK_TYPE.into(), agent_address.to_string())?;

    Ok(())
}

pub fn agents_root_entry_def() -> ValidatingEntryType {
    entry!(
        name: AGENT_ROOT_ANCHOR_ID,
        description: "Root anchor for querying all registered agents in the network",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<String>| {
            match validation_data {
                EntryValidationData::Create { .. } => {
                    Ok(())
                },
                _ => Err(String::from("Cannot update or delete root agent anchor")),
            }
        },
        links: [
            to!(
                AGENT_ANCHOR_ID,
                link_type: AGENT_ANCHOR_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | validation_data: hdk::LinkValidationData | {
                    match validation_data {
                        hdk::LinkValidationData::LinkAdd { .. } => Ok(()),
                        _ => Err(String::from("Cannot delete agent registration links (mark the agent deleted instead)"))
                    }
                }
            )
        ]
    )
}
