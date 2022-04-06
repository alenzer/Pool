use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};
use crate::msg::{UserInfo, AprInfo, PayRequest, AmountInfo};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const TREASURY: Item<Addr> = Item::new("treasury wallet");

pub const UST_APR_HISTORY: Item<Vec<AprInfo>> = Item::new("history of ust apr");
pub const UST_USER_INFOS: Map<Addr, UserInfo> = Map::new("UST user infos");
pub const UST_REWARDS_REQUEST: Item<Vec<PayRequest>> = Item::new("ust reward pay request");
pub const UST_WITHDRAW_REQUEST: Item<Vec<PayRequest>> = Item::new("ust withdraw request");

pub const LUNA_APR_HISTORY: Item<Vec<AprInfo>> = Item::new("history of luna apr");
pub const LUNA_USER_INFOS: Map<Addr, UserInfo> = Map::new("LUNA user infos");
pub const LUNA_REWARDS_REQUEST: Item<Vec<PayRequest>> = Item::new("LUNA reward pay request");
pub const LUNA_WITHDRAW_REQUEST: Item<Vec<PayRequest>> = Item::new("LUNA withdraw request");

pub const AMOUNT_HISTORY: Item<Vec<AmountInfo>> = Item::new("Amount history");