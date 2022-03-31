use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Map, U128Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub treasury: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig {
        owner: Option<Addr>,
        treasury: Option<Addr>,
    },
    SetUSTAPR{
        apr: Uint128
    },
    DepositUST {
    },
    WithdrawUST {
        amount: Uint128,
    },
    ClaimRewardsUST{
        wallet: Addr,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner{ },
    GetHistoryOfAPR{ },
    GetUserInfoUST{ wallet: Addr },
    GetPendingRewardsUST{ wallet: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AprInfo{
	pub apr: Uint128,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub wallet: Addr,
	pub amount: Uint128,
	pub reward_amount: Uint128,
    pub last_reward_time: Uint128,
}
