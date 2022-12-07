use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

/// ## Description
/// This structure holds the main parameters for the of generator_proxy_to_mirror contract.
#[cw_serde]
pub struct Config {
    /// The Generator contract address
    pub generator_contract_addr: Addr,
    /// The target Paloma pair contract address
    pub pair_addr: Addr,
    /// The contract address for the Paloma MIR LP token
    pub lp_token_addr: Addr,
    /// The 3rd party reward contract address
    pub reward_contract_addr: Addr,
    /// The 3rd party reward token
    pub reward_token_addr: Addr,
}

/// ## Description
/// Stores the contract config at the given key
pub const CONFIG: Item<Config> = Item::new("config");
