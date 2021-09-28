/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package agent registration zome
 * @author  pospi <pospi@spadgos.com>
 * @since   2020-03-22
 */

pub const AGENT_ROOT_ANCHOR_ID: &str = "registered_agents_root";
pub const AGENT_ANCHOR_LINK_TYPE: &str = "registered_agent";
pub const AGENT_ANCHOR_ID: &str = "%agent_id";
