use hdk::prelude::*;

use hc_zome_agent_registration_lib::*;
use hc_zome_agent_registration_rpc::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
    ]))
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    init_agent_registration_storage()?;
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn validate(validation_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    validate_registration_path(validation_data)
}

#[hdk_extern]
fn is_registered(RegistrationQueryParams { pub_key }: RegistrationQueryParams) -> ExternResult<bool> {
    is_registered_agent(&pub_key)
}

#[hdk_extern]
fn get_registered(_: ()) -> ExternResult<Vec<AgentPubKey>> {
    get_registered_agents()
}
