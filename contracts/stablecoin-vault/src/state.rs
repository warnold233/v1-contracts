use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::Item;


use crate::pool_info::PoolInfoRaw;

pub static LUNA_DENOM: &str = "uluna";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
    pub trader: CanonicalAddr,
    pub pool_address: CanonicalAddr,
    pub anchor_money_market_address: CanonicalAddr,
    pub aust_address: CanonicalAddr,
    pub seignorage_address: CanonicalAddr,
    pub profit_check_address: CanonicalAddr,
}

pub const STATE: Item<State> = Item::new("\u{0}{5}state");
pub const POOL_INFO: Item<PoolInfoRaw> = Item::new("\u{0}{4}pool");