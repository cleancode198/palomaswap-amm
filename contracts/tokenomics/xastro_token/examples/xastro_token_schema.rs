use cosmwasm_schema::write_api;

use cw20_base::msg::ExecuteMsg;
use paloma::xastro_token::{InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg
    }
}
