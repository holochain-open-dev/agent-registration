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
use hdk::hash_path::path::Component;

use hc_zome_agent_registration_storage::{path_for_agent, get_root_anchor};

/// Returns true if the given agent ID is a member of the local DNA
pub fn is_registered_agent(address: &AgentPubKey) -> ExternResult<bool> {
    path_for_agent(address).exists()
}

/// Returns the addresses of all agents who have accessed the local DNA
pub fn get_registered_agents() -> ExternResult<Vec<AgentPubKey>> {
    let child_paths = get_root_anchor().children()?;
    debug!("TODO CHILDRENS {:?}", child_paths);
    Ok(vec![])
}

/// Initialises data needed to query registered agents from the network.
/// Must be called at zome init time.
pub fn init_agent_registration_storage() -> ExternResult<()>
{
    let agent_address = agent_info()?.agent_latest_pubkey;

    // Avoid duplicate linking if already registered
    if Ok(true) == is_registered_agent(&agent_address) {
        return Ok(());
    }

    // Not already registered- wire up paths & (implicitly) link them
    let root = get_root_anchor();
    root.ensure()?;
    let agent_path = path_for_agent(&agent_address);
    agent_path.ensure()?;

    Ok(())
}

/// Checks for and validates any creation of an agent address path
///
pub fn validate_registration_path(validation_data: ValidateData) -> ExternResult<ValidateCallbackResult>
{
    let element = validation_data.element;
    let (signed_header, entry) = element.into_inner();
    let entry = match entry {
        ElementEntry::Present(e) => e,
        _ => return Ok(ValidateCallbackResult::Valid),
    };

    let root_path = get_root_anchor();

    match Path::try_from(&entry) {
        Ok(any_path) => {
            // if the path is rooted in the registration anchor, ensure its creator is the signee
            if any_path.parent() == Some(root_path) {
                return validate_path_agent_matches(&any_path, &signed_header);
            }

            Ok(ValidateCallbackResult::Valid)  // not correct type of Path
        },
        _ => Ok(ValidateCallbackResult::Valid), // not a Path
    }
}

/// Ensure that the trailing `Component` of a `Path` matches the `AgentPubKey` of the agent signing some header
///
fn validate_path_agent_matches(path_with_agent_suffix: &Path, signed_header: &SignedHeaderHashed) -> ExternResult<ValidateCallbackResult>
{
    let components: &Vec<Component> = path_with_agent_suffix.as_ref();
    let last = components.as_slice().last().ok_or(
        WasmError::Guest("agent registration Path of invalid length".to_string())
    )?;
    let written_agent_pubkey = AgentPubKey::from_raw_39(last.as_ref().to_vec()).map_err(|_e| {
        WasmError::Guest(format!("agent registration has invalid pubKey {:?}", last))
    })?;

    verify_signature(written_agent_pubkey, signed_header.signature().to_owned(), signed_header.header())?;

    Ok(ValidateCallbackResult::Valid)
}
