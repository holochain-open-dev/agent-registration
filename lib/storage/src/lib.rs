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
};

use hc_zome_agent_registration_storage_consts::*;
use hc_zome_agent_registration_lib::{get_root_entry, get_root_entry_address};

/// Initialises data needed to query registered agents from the network.
/// Must be called at zome init time.
pub fn init() -> Result<(), String> {
    commit_entry(&get_root_entry())?;
    Ok(())
}

/// Register new agents with the DNA as they join. Should reject registration on any errors.
/// Must be called from validate_agent callback.
pub fn handle_agent_registration(validation_data: EntryValidationData<AgentId>) -> Result<(), String> {
    match validation_data {
        EntryValidationData::Create {
            validation_data, ..
        } => {
            let agent_address = validation_data.package.chain_header.entry_address();
            let root_address = get_root_entry_address()?;

            link_entries(&root_address, agent_address, AGENT_ANCHOR_LINK_TYPE.into(), "")?;

            Ok(())
        }
        // :TODO: unsure what EntryValidationData::Modify does for agent
        // :TODO: how to handle handle de-registration / deletion? Does this work as expected?
        _ => Ok(())
    }
}

pub fn agents_root_entry_def() -> ValidatingEntryType {
    entry!(
        name: AGENT_ROOT_ANCHOR_ID,
        description: "Root anchor for querying all registered agents in the network",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<Address>| {
            match validation_data {
                EntryValidationData::Create { .. } => {
                    Ok(())
                },
                _ => Err(String::from("Cannot update or delete root agent anchor")),
            }
        },
        links: [
            from!(
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
