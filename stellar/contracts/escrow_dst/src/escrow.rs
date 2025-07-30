use crate::timelocks::{Timelocks, TimelocksLib};
use soroban_sdk::token::TokenClient;
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, U256};


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    AccessToken,
    RescueDelay,
    XLMToken,
}

#[derive(Clone)]
#[contracttype]
pub struct Immutables {
    pub order_hash: BytesN<32>, // bytes32 orderHash
    pub hashlock: BytesN<32>,   // bytes32 hashlock - Hash of the secret
    pub maker: Address,         // Address maker
    pub taker: Address,         // Address taker
    pub token: Address,         // Address token
    pub amount: U256,           // uint256 amount
    pub safety_deposit: U256,   // uint256 safetyDeposit
    pub timelocks: Timelocks,   // Timelocks timelocks
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn constructor(env: Env, access_token: Address, rescue_delay: U256) {
        env.storage()
            .instance()
            .set(&DataKey::AccessToken, &access_token);
        env.storage()
            .instance()
            .set(&DataKey::RescueDelay, &rescue_delay);
    }

    // Helper functions to retrieve stored values
    pub fn get_access_token(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::AccessToken)
            .unwrap_or_else(|| panic!("Access token not found"))
    }

    pub fn get_rescue_delay(env: Env) -> U256 {
        env.storage()
            .instance()
            .get(&DataKey::RescueDelay)
            .unwrap_or_else(|| panic!("Rescue delay not found"))
    }

    pub fn rescue_funds(
        env: Env,
        token: Address,
        amount: U256, // Changed from u128 to U256 to match your type usage
        immutables: Immutables,
    ) {
        // Check authorization
        Self::only_taker(&env, &immutables);

        // Get rescue delay from storage and convert to U256
        let rescue_delay = Self::get_rescue_delay(env.clone());

        // Calculate rescue start time using timelocks
        let rescue_start_time =
            TimelocksLib::rescue_start(immutables.timelocks.clone(), rescue_delay);

        // Check if enough time has passed
        Self::only_after(&env, rescue_start_time);

        // Transfer the tokens
        Self::uni_transfer(&env, &token, &immutables.taker, &amount);

        // Emit event
        env.events().publish((token, amount), "funds_rescued");
    }

    pub(crate) fn only_taker(_env: &Env, immutables: &Immutables) {
        immutables.taker.require_auth();
    }

    pub(crate) fn only_after(env: &Env, timelock: U256) {
        // Convert timestamp to U256 for comparison
        let current_timestamp = env.ledger().timestamp();
        let current_time = U256::from_u128(env, current_timestamp.into());

        // Check if current time is before the timelock
        if current_time < timelock {
            panic!("Too early");
        }
    }

    pub(crate) fn only_before(env: &Env, timelock: U256) {
        // Convert timestamp to U256 for comparison
        let current_timestamp = env.ledger().timestamp();
        let current_time = U256::from_u128(env, current_timestamp.into());

        // Check if current time is at or after the timelock
        if current_time >= timelock {
            panic!("Too late");
        }
    }

    pub(crate) fn only_valid_secret(env: &Env, secret: &BytesN<32>, immutables: &Immutables) {
        let hash = env.crypto().keccak256(&secret.clone().into());
        if BytesN::from(hash) != immutables.hashlock {
            panic!("Invalid secret provided");
        }
    }

    pub(crate) fn uni_transfer(env: &Env, token: &Address, to: &Address, amount: &U256) {
        // Convert U256 to i128 for Soroban token transfer
        let amount_u128 = amount
            .to_u128()
            .unwrap_or_else(|| panic!("Amount overflow"));
        let amount_i128 = amount_u128 as i128;
        // Create token client and transfer
        let token_client = TokenClient::new(env, token);
        token_client.transfer(&env.current_contract_address(), to, &amount_i128);
    }

    pub(crate) fn _withdraw(env: &Env, secret: BytesN<32>, immutables: &Immutables) {
        // Validate secret
        Self::only_valid_secret(env, &secret, immutables);

        // Transfer tokens to maker
        Self::uni_transfer(
            env,
            &immutables.token,
            &immutables.maker,
            &immutables.amount,
        );

        // Transfer safety deposit
        Self::uni_transfer(
            env,
            &immutables.token, // Using same token for safety deposit in Soroban
            &immutables.taker,
            &immutables.safety_deposit,
        );

        // Emit withdrawal event
        env.events().publish(((secret),), "funds withdrawn");
    }
}