#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

#[zome]
mod agent_registration_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        match validation_data {
            EntryValidationData::Create {
                validation_data, ..
            } => {
								let _agent_address = validation_data.package.chain_header.entry_address();
								// :TODO: write entry
                Ok(())
            }
            _ => Err(String::from("Error validating the agent")),
        }
    }

    // :TODO:
}
