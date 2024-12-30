use serde::{Deserialize, Serialize};
use cosmwasm_std::Decimal;
use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub token_name: String,
    pub symbol: String,
    pub denom: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub initial_circulating_supply: u128,
    pub distribution: Distribution,
    pub inflation: Inflation,
    pub governance: Governance,
    pub staking: Staking,
    pub liquidity_pools: Vec<LiquidityPool>,
    pub ibc_channels: Vec<IbcChannel>,
}

#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Distribution {
    pub validators: u128,
    pub community_reserve: u128,
    pub staking_rewards: u128,
    pub liquidity_provision: u128,
    pub foundation: u128,
}

#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Inflation {
    pub rate: String,
    pub decay_rate: f64,
    pub decay_time: String,
    pub min_inflation: String,
    pub capped: bool,
}

#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Governance {
    pub staking_requirement: String,
    pub voting_power: String,
    pub quorum: String,
    pub threshold: String,
}

#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Staking {
    pub unbonding_time: String,
    pub max_validators: u32,
    pub reward_distribution: String,
}

#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LiquidityPool {
    pub pair: String,
    pub pool_type: String,
}

#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IbcChannel {
    pub target_chain: String,
    pub transfer_enabled: bool,
}
