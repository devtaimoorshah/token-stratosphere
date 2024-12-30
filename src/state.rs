use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

// Define the token balances for users
const TOKENS: Map<&Addr, Token> = Map::new("tokens");

// Define the Token struct to hold the amount
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Token {
    pub amount: u128,
}

// Initialize TokenConfig item for storing token configuration
const TOKEN_CONFIG: Item<TokenConfig> = Item::new("token_config");
