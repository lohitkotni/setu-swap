#![no_std]
mod timelocks;
mod escrow;

use soroban_sdk::{contract, contractimpl, Env, Address, U256, BytesN, contracttype};

pub use timelocks::{Stage,TimelocksLib, Timelocks};
pub use escrow::{EscrowContract,Immutables};

#[contract]
pub struct EscrowSrc;

#[contractimpl]
impl EscrowSrc {
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
        target: Address
    ) {
        // Authorization check
        escrow::EscrowContract::only_taker(&env, &immutables);
    
        // Time window checks
        let withdrawal_time = TimelocksLib::get(&env, immutables.timelocks.clone(), Stage::SrcWithdrawal as u32);
        let cancellation_time = TimelocksLib::get(&env, immutables.timelocks.clone(), Stage::SrcCancellation as u32);
        
        escrow::EscrowContract::only_after(&env, withdrawal_time);
        escrow::EscrowContract::only_before(&env, cancellation_time);
    
        // Withdraw to the sender (equivalent to msg.sender)
        Self::_withdraw_to(&env, secret, &target, &immutables);
    }

    pub fn withdraw_to(
        env: Env,
        secret: BytesN<32>,
        target: Address,
        immutables: Immutables,
    ) {
        // Authorization check
        escrow::EscrowContract::only_taker(&env, &immutables);
    
        // Time window checks
        let withdrawal_time = TimelocksLib::get(&env, immutables.timelocks.clone(), Stage::SrcWithdrawal as u32);
        let cancellation_time = TimelocksLib::get(&env, immutables.timelocks.clone(), Stage::SrcCancellation as u32);
        
        escrow::EscrowContract::only_after(&env, withdrawal_time);
        escrow::EscrowContract::only_before(&env, cancellation_time);
    
        // Withdraw to specified target address
        Self::_withdraw_to(&env, secret, &target, &immutables);
    }

    pub fn public_withdraw(
        env: Env,
        secret: BytesN<32>,
        immutables: Immutables,
    ) {
        // Time window checks
        let public_withdrawal_time = TimelocksLib::get(
            &env, 
            immutables.timelocks.clone(), 
            Stage::SrcPublicWithdrawal as u32
        );
        let cancellation_time = TimelocksLib::get(
            &env, 
            immutables.timelocks.clone(), 
            Stage::SrcCancellation as u32
        );
        
        // Verify time constraints
        escrow::EscrowContract::only_after(&env, public_withdrawal_time);
        escrow::EscrowContract::only_before(&env, cancellation_time);
    
        // Withdraw to the taker's address
        Self::_withdraw_to(&env, secret, &immutables.taker, &immutables);
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
            Stage::SrcCancellation as u32
        );
        escrow::EscrowContract::only_after(&env, cancellation_time);
    
        // Perform cancellation
        Self::_cancel(&env, &immutables);
    }

    pub fn public_cancel(
        env: Env,
        immutables: Immutables,
    ) {
    
        // Time window check for public cancellation
        let public_cancellation_time = TimelocksLib::get(
            &env, 
            immutables.timelocks.clone(), 
            Stage::SrcPublicCancellation as u32
        );
        escrow::EscrowContract::only_after(&env, public_cancellation_time);
    
        // Perform cancellation
        Self::_cancel(&env, &immutables);
    }

    pub(crate) fn _withdraw_to(
        env: &Env,
        secret: BytesN<32>,
        target: &Address,
        immutables: &Immutables,
    ) {
        escrow::EscrowContract::only_valid_secret(env, &secret, immutables);
    
        // Transfer tokens to target
        escrow::EscrowContract::uni_transfer(
            env,
            &immutables.token,
            target,
            &immutables.amount
        );
    
        // Transfer safety deposit to the caller
        escrow::EscrowContract::uni_transfer(
            env,
            &immutables.token,
            &immutables.taker,
            &immutables.safety_deposit
        );
    
        // Emit withdrawal event
        env.events().publish(
            ("escrow_withdrawal", secret),
            ()
        );
    }

    pub(crate) fn _cancel(
        env: &Env,
        immutables: &Immutables,
    ) {
        // Transfer tokens back to maker (equivalent to IERC20(token).safeTransfer(maker, amount))
        escrow::EscrowContract::uni_transfer(
            env,
            &immutables.token,
            &immutables.maker,
            &immutables.amount
        );
    
        // Transfer safety deposit to caller (equivalent to _ethTransfer(msg.sender, safetyDeposit))
        escrow::EscrowContract::uni_transfer(
            env,
            &immutables.token,
            &immutables.taker,  // equivalent to msg.sender
            &immutables.safety_deposit
        );
    
        // Emit cancellation event (equivalent to emit EscrowCancelled())
        env.events().publish(
            ("escrow_cancelled",),
            ()
        );
    }
}