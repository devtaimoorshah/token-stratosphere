use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

// Define Token struct to hold the amount
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct Token {
    pub amount: u128,
}

// Define a TOKENS Item for storing user balances
const TOKENS: Item<Token> = Item::new("tokens");

// This function is used to transfer tokens from sender to recipient
pub fn transfer_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient_addr: Addr,
    amount: u128,
) -> StdResult<Response> {
    let sender_addr = info.sender.clone();  // Clone the sender's address

    // Load sender's balance from storage, default to 0 if not found
    let mut sender_balance = TOKENS.load(deps.storage) // Only pass deps.storage
        .unwrap_or_else(|_| Token { amount: 0 });

    // Check if the sender has enough balance
    if sender_balance.amount < amount {
        return Err(StdError::generic_err("Insufficient funds"));
    }

    // Deduct the amount from the sender's balance
    sender_balance.amount -= amount;

    // Load recipient's balance from storage, default to 0 if not found
    let mut recipient_balance = TOKENS.load(deps.storage) // Only pass deps.storage
        .unwrap_or_else(|_| Token { amount: 0 });

    // Add the amount to the recipient's balance
    recipient_balance.amount += amount;

    // Save the updated balances back to storage
    TOKENS.save(deps.storage, &sender_balance)?;  // Only save the data (no key needed)
    TOKENS.save(deps.storage, &recipient_balance)?;  // Only save the data (no key needed)

    // Return the response with the "transfer" action
    Ok(Response::new().add_attribute("action", "transfer"))
}

// Optional: Define an "init" function to initialize balances or other contract state
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> StdResult<Response> {
    // Initialize any contract state or perform setup logic
    Ok(Response::new().add_attribute("action", "instantiate"))
}

// Optional: Define an "execute" function to handle various contract actions
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient_addr: Addr,
    amount: u128,
) -> StdResult<Response> {
    // Check if the action is 'transfer'
    if info.sender == recipient_addr {
        return Err(StdError::generic_err("Cannot transfer to oneself"));
    }
    // Call the transfer_tokens function to transfer the specified amount
    transfer_tokens(deps, env, info, recipient_addr, amount)
}
