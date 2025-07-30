#![no_std]
mod timelocks;
mod escrow;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, U256};

pub use escrow::{EscrowContract, Immutables};
pub use timelocks::{Stage, Timelocks, TimelocksLib};

#[contract]
pub struct EscrowDst;

#[contractimpl]
impl EscrowDst {
    pub fn __constructor(env: Env, access_token: Address, rescue_delay: U256) {
        EscrowContract::constructor(env, access_token, rescue_delay);
    }

    pub fn withdraw(env: Env, secret: BytesN<32>, immutables: Immutables) {
        // Authorization check
        EscrowContract::only_taker(&env, &immutables);

        // Time window checks
        let withdrawal_time = TimelocksLib::get(
            &env,
            immutables.timelocks.clone(),
            Stage::DstWithdrawal as u32,
        );
        let cancellation_time = TimelocksLib::get(
            &env,
            immutables.timelocks.clone(),
            Stage::DstCancellation as u32,
        );

        EscrowContract::only_after(&env, withdrawal_time);
        EscrowContract::only_before(&env, cancellation_time);

        // Perform withdrawal
        EscrowContract::_withdraw(&env, secret, &immutables);
    }

    pub fn cancel(env: Env, immutables: Immutables) {
        // Authorization check
        EscrowContract::only_taker(&env, &immutables);

        // Time window check for cancellation
        let cancellation_time = TimelocksLib::get(
            &env,
            immutables.timelocks.clone(),
            Stage::DstCancellation as u32,
        );
        EscrowContract::only_after(&env, cancellation_time);

        // Transfer tokens back to taker
        EscrowContract::uni_transfer(
            &env,
            &immutables.token,
            &immutables.taker,
            &immutables.amount,
        );

        // Transfer safety deposit
        EscrowContract::uni_transfer(
            &env,
            &immutables.token,
            &immutables.taker,
            &immutables.safety_deposit,
        );

        // Emit cancellation event
        env.events().publish(("escrow_cancelled",), ());
    }

    pub fn public_withdraw(env: Env, secret: BytesN<32>, immutables: Immutables) {
        // Check if caller is access token holder
        let access_token = EscrowContract::get_access_token(env.clone());
        access_token.require_auth();

        // Time window checks
        let public_withdrawal_time = TimelocksLib::get(
            &env,
            immutables.timelocks.clone(),
            Stage::DstPublicWithdrawal as u32,
        );
        let cancellation_time = TimelocksLib::get(
            &env,
            immutables.timelocks.clone(),
            Stage::DstCancellation as u32,
        );

        // Verify time constraints
        EscrowContract::only_after(&env, public_withdrawal_time);
        EscrowContract::only_before(&env, cancellation_time);

        // Perform withdrawal
        EscrowContract::_withdraw(&env, secret, &immutables);
    }
}
