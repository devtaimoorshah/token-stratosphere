use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use cosmwasm_std::entry_point;  // Add this import

// Token configuration structure
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TokenConfig {
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

// Message for instantiating the contract (received from the transaction)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Distribution {
    pub validators: u128,
    pub community_reserve: u128,
    pub staking_rewards: u128,
    pub liquidity_provision: u128,
    pub foundation: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Inflation {
    pub rate: String,
    pub decay_rate: f64,
    pub decay_time: String,
    pub min_inflation: String,
    pub capped: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Governance {
    pub staking_requirement: String,
    pub voting_power: String,
    pub quorum: String,
    pub threshold: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Staking {
    pub unbonding_time: String,
    pub max_validators: u32,
    pub reward_distribution: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LiquidityPool {
    pub pair: String,
    pub pool_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IbcChannel {
    pub target_chain: String,
    pub transfer_enabled: bool,
}

// Define the token balances for users
const TOKENS: Map<&Addr, Token> = Map::new("tokens");

// Define the Token struct to hold the amount
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Token {
    pub amount: u128,
}

// Initialize TokenConfig item for storing token configuration
const TOKEN_CONFIG: Item<TokenConfig> = Item::new("token_config");

// Instantiate function to set up the token configuration
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,  // Use the InstantiateMsg here
) -> StdResult<Response> {
    // Log the incoming message for debugging
    log::info!("Received instantiate message: {:?}", msg);

    let token_config = TokenConfig {
        token_name: msg.token_name,
        symbol: msg.symbol,
        denom: msg.denom,
        decimals: msg.decimals,
        total_supply: msg.total_supply,
        initial_circulating_supply: msg.initial_circulating_supply,
        distribution: msg.distribution,
        inflation: msg.inflation,
        governance: msg.governance,
        staking: msg.staking,
        liquidity_pools: msg.liquidity_pools,
        ibc_channels: msg.ibc_channels,
    };

    TOKEN_CONFIG.save(deps.storage, &token_config)?;

    // Initialize the total supply for the contract
    let initial_balance = token_config.initial_circulating_supply;
    let owner_addr = info.sender.clone();
    TOKENS.save(deps.storage, &owner_addr, &Token { amount: initial_balance })?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner_addr)
        .add_attribute("total_supply", initial_balance.to_string()))
}

// Transfer function to allow users to send tokens to each other
pub fn transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: Addr,
    amount: u128,
) -> StdResult<Response> {
    let sender_addr = info.sender.clone();
    let sender_balance = TOKENS.load(deps.storage, &sender_addr)?;

    if sender_balance.amount < amount {
        return Err(StdError::generic_err("Insufficient balance"));
    }

    // Update balances
    TOKENS.save(deps.storage, &sender_addr, &Token { amount: sender_balance.amount - amount })?;

    let recipient_balance = TOKENS.load(deps.storage, &recipient).unwrap_or_default();
    TOKENS.save(deps.storage, &recipient, &Token { amount: recipient_balance.amount + amount })?;

    Ok(Response::new()
        .add_attribute("method", "transfer")
        .add_attribute("from", sender_addr)
        .add_attribute("to", recipient)
        .add_attribute("amount", amount.to_string()))
}

// Query function to check the balance of a user
pub fn query_balance(deps: DepsMut, address: Addr) -> StdResult<u128> {
    let balance = TOKENS.load(deps.storage, &address)?;
    Ok(balance.amount)
}
