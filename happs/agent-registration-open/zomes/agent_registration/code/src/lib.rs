#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_agent_registration_storage;
use hc_zome_agent_registration_lib;

#[zome]
mod agent_registration_zome {
    #[init]
    fn init() {
        hc_zome_agent_registration_storage::init()
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        hc_zome_agent_registration_storage::handle_agent_registration(validation_data)
    }

    #[entry_def]
    pub fn agents_root_entry_def() -> ValidatingEntryType {
        hc_zome_agent_registration_storage::agents_root_entry_def()
    }

    #[zome_fn("hc_public")]
    pub fn is_registered_agent(address: Address) -> ZomeApiResult<bool> {
        hc_zome_agent_registration_lib::is_registered_agent(address)
    }

    #[zome_fn("hc_public")]
    pub fn get_registered_agents() -> ZomeApiResult<Vec<Address>> {
        hc_zome_agent_registration_lib::get_registered_agents()
    }
}
