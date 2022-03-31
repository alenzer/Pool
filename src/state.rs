use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};
use crate::msg::{UserInfo, AprInfo};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const TREASURY: Item<Addr> = Item::new("treasury wallet");
pub const UST_APR_HISTORY: Item<Vec<AprInfo>> = Item::new("history of apr");
pub const UST_USER_INFOS: Map<Addr, UserInfo> = Map::new("UST user infos");

pub const LUNA_APR_HISTORY: Item<Vec<Uint128>> = Item::new("history of apr");
pub const LUNA_USER_INFOS: Map<Addr, UserInfo> = Map::new("LUNA user infos");
