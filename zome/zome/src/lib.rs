use hdk::prelude::*;

use hc_zome_agent_registration_lib::*;
use hc_zome_agent_registration_rpc::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
    ]))
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    init_agent_registration_storage()?;
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op {
        Op::StoreElement { .. } => Ok(ValidateCallbackResult::Valid),
        Op::StoreEntry { entry, header } => validate_registration_path(entry, header),
        Op::RegisterCreateLink { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDeleteLink { .. } => Ok(ValidateCallbackResult::Invalid("cannot delete registered agent links".to_string())),
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Invalid("cannot update registered agent information".to_string())),
        Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Invalid("cannot delete registered agent data".to_string())),
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }
}

#[hdk_extern]
fn is_registered(RegistrationQueryParams { pub_key }: RegistrationQueryParams) -> ExternResult<bool> {
    is_registered_agent(&pub_key)
}

#[hdk_extern]
fn get_registered(_: ()) -> ExternResult<Vec<AgentPubKey>> {
    get_registered_agents()
}

#[hdk_extern]
fn get_my_agent_pubkey(_: ()) -> ExternResult<AgentPubKey> {
    Ok(agent_info()?.agent_latest_pubkey)
}
