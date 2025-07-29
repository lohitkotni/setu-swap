#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, U256, BytesN, contracttype};

use crate::timelocks::{Timelocks, Stage,TimelocksLib};
use crate::escrow::{EscrowContract,Immutables};

#[contract]
pub struct EscrowDst;

#[contractimpl]
impl EscrowDst {
    pub fn __constructor(
        env: Env,
        access_token: Address,
        rescue_delay: U256,
    ) {
        escrow::EscrowContract::constructor(env, access_token, rescue_delay);
    }

    pub fn withdraw(
        env: Env,
        secret: BytesN<32>,
        immutables: Immutables,
    ) {
        // Authorization check
        escrow::EscrowContract::only_taker(&env, &immutables);

        // Time window checks
        let withdrawal_time = TimelocksLib::get(&env, immutables.timelocks.clone(), Stage::DstWithdrawal as u32);
        let cancellation_time = TimelocksLib::get(&env, immutables.timelocks.clone(), Stage::DstCancellation as u32);
        
        escrow::EscrowContract::only_after(&env, withdrawal_time);
        escrow::EscrowContract::only_before(&env, cancellation_time);

        // Perform withdrawal
        escrow::EscrowContract::_withdraw(&env, secret, &immutables);
    }

    pub fn cancel(
        env: Env,
        immutables: Immutables,
    ) {
        // Authorization check
        escrow::EscrowContract::only_taker(&env, &immutables);

        // Time window check for cancellation
        let cancellation_time = TimelocksLib::get(
            &env, 
            immutables.timelocks.clone(), 
            Stage::DstCancellation as u32
        );
        escrow::EscrowContract::only_after(&env, cancellation_time);

        // Transfer tokens back to taker
        escrow::EscrowContract::uni_transfer(
            &env,
            &immutables.token,
            &immutables.taker,
            &immutables.amount
        );

        // Transfer safety deposit
        escrow::EscrowContract::uni_transfer(
            &env,
            &immutables.token,
            &immutables.taker,
            &immutables.safety_deposit
        );

        // Emit cancellation event
        env.events().publish(
            ("escrow_cancelled",),
            ()
        );
    }

    pub fn public_withdraw(
        env: Env,
        secret: BytesN<32>,
        immutables: Immutables,
    ) {
        // Check if caller is access token holder
        let access_token = escrow::EscrowContract::get_access_token(env.clone());
        access_token.require_auth();

        // Time window checks
        let public_withdrawal_time = TimelocksLib::get(
            &env, 
            immutables.timelocks.clone(), 
            Stage::DstPublicWithdrawal as u32
        );
        let cancellation_time = TimelocksLib::get(
            &env, 
            immutables.timelocks.clone(), 
            Stage::DstCancellation as u32
        );
        
        // Verify time constraints
        escrow::EscrowContract::only_after(&env, public_withdrawal_time);
        escrow::EscrowContract::only_before(&env, cancellation_time);

        // Perform withdrawal
        escrow::EscrowContract::_withdraw(&env, secret, &immutables);
    }  
}