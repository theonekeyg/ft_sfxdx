use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

use cosmwasm_std::{HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitBalance {
    pub address: HumanAddr,
    pub amount: Uint128
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {
    pub balances: Option<Vec<InitBalance>>
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Transfer {
        to: HumanAddr,
        amount: Uint128
    },
    Burn {
        amount: Uint128
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: HumanAddr }
}
