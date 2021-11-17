use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, Empty, StdResult, Uint128, WasmMsg};

use crate::treasury::vault_assets::VaultAsset;
use terraswap::asset::AssetInfo;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetAdmin {
        admin: String,
    },
    TraderAction {
        msgs: Vec<CosmosMsg<Empty>>,
    },
    AddTrader {
        trader: String,
    },
    RemoveTrader {
        trader: String,
    },
    UpdateAssets {
        to_add: Vec<VaultAsset>,
        to_remove: Vec<AssetInfo>,
    },
    // UpdateBalances {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    TotalValue {},
    HoldingValue { identifier: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub traders: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalValueResponse {
    pub value: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HoldingValueResponse {
    pub value: Uint128,
}

pub fn send_to_treasury(
    msgs: Vec<CosmosMsg>,
    treasury_address: &Addr,
) -> StdResult<CosmosMsg<Empty>> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: treasury_address.to_string(),
        msg: to_binary(&ExecuteMsg::TraderAction { msgs })?,
        funds: vec![],
    }))
}
