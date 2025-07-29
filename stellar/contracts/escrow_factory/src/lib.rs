#![no_std]
mod escrow_src;
mod escrow_dst;
mod escrow;
mod timelocks;

use soroban_sdk::{contract, contractimpl, contracttype, Env, U256, BytesN, Address, token::TokenClient};
pub use escrow_src::EscrowSrc;
pub use escrow_dst::EscrowDst;
pub use escrow::{Immutables, EscrowContract};
pub use timelocks::{Timelocks, TimelocksLib, Stage};

#[contract]
pub struct EscrowFactory;

#[contractimpl]
impl EscrowFactory {
    pub fn __constructor(
        env: Env,
        rescue_delay_src: U256,
        rescue_delay_dst: U256,
    ) {
        env.storage().instance().set(&DataKey::RescueDelaySrc, &rescue_delay_src);
        env.storage().instance().set(&DataKey::RescueDelayDst, &rescue_delay_dst);
    }

    pub fn create_src_escrow(
        env: Env,
        immutables: Immutables,
    ) {
        let mut updated_immutables = immutables.clone();
        updated_immutables.timelocks = TimelocksLib::set_deployed_at(
            &env,
            updated_immutables.timelocks,
            env.ledger().timestamp().into(),
        );

        let escrow_address = escrow_src::EscrowSrc::new(
            &env,
            updated_immutables.clone(),
            env.storage().instance().get(&DataKey::RescueDelaySrc).unwrap(),
        );

        // Transfer tokens (amount + safety deposit) to escrow
        let total_amount = immutables.amount.add(&immutables.safety_deposit);
        let amount_u128 = total_amount.to_u128().unwrap_or_else(|| panic!("Amount overflow"));
        let amount_i128 = amount_u128 as i128;
        let token_client = TokenClient::new(&env, &immutables.token);
        token_client.transfer(&env.invoker(), &escrow_address, &amount_i128);

        env.events().publish(
            (
                "src_escrow_created",
                escrow_address,
                immutables.hashlock,
                immutables.maker,
                immutables.taker,
            ),
            (),
        );
    }

    pub fn create_dst_escrow(
        env: Env,
        dst_immutables: Immutables,
        src_cancellation_timestamp: U256,
    ) {
        let mut updated_immutables = dst_immutables.clone();
        updated_immutables.timelocks = TimelocksLib::set_deployed_at(
            &env,
            updated_immutables.timelocks,
            env.ledger().timestamp().into(),
        );

        let dst_cancellation = TimelocksLib::get(
            &env,
            updated_immutables.timelocks.clone(),
            Stage::DstCancellation as u32,
        );
        
        // Validate creation timing
        if dst_cancellation > src_cancellation_timestamp {
            panic!("Invalid creation time");
        }

        let escrow_address = escrow_dst::EscrowDst::new(
            &env,
            updated_immutables.clone(),
            env.storage().instance().get(&DataKey::RescueDelayDst).unwrap(),
        );

        // Transfer tokens (amount + safety deposit) to escrow
        let total_amount = dst_immutables.amount.add(&dst_immutables.safety_deposit);
        let amount_u128 = total_amount.to_u128().unwrap_or_else(|| panic!("Amount overflow"));
        let amount_i128 = amount_u128 as i128;
        let token_client = TokenClient::new(&env, &dst_immutables.token);
        token_client.transfer(&env.invoker(), &escrow_address, &amount_i128);

        env.events().publish(
            (
                "dst_escrow_created",
                escrow_address,
                dst_immutables.hashlock,
                dst_immutables.taker,
            ),
            (),
        );
    }
}

#[contracttype]
pub enum DataKey {
    RescueDelaySrc,
    RescueDelayDst,
}