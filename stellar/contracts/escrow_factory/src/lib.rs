// #![no_std]
// mod escrow;
// mod escrow_dst;
// mod escrow_src;
// mod timelocks;

// use soroban_sdk::{
//     Val,contract, contractimpl, symbol_short, token::TokenClient, Address, BytesN, Env,
//     IntoVal,Vec, U256, Symbol,
// };

// pub use escrow::{EscrowContract, Immutables};
// pub use timelocks::{Stage, Timelocks, TimelocksLib};

// #[contract]
// pub struct EscrowFactory;

// const RESCUE_DELAY_SRC: Symbol = symbol_short!("res_src");
// const RESCUE_DELAY_DST: Symbol = symbol_short!("res_dst");

// #[contractimpl]
// impl EscrowFactory {
//     pub fn __constructor(env: Env, rescue_delay_src: U256, rescue_delay_dst: U256) {
//         env.storage()
//             .instance()
//             .set(&RESCUE_DELAY_SRC, &rescue_delay_src);
//         env.storage()
//             .instance()
//             .set(&RESCUE_DELAY_DST, &rescue_delay_dst);
//     }

//     pub fn create_src_escrow(
//         env: Env,
//         wasm_hash: BytesN<32>,
//         salt: BytesN<32>,
//         mut immutables: Immutables,
//     ) -> Address {
//         let current_timestamp = env.ledger().timestamp();
//         let current_time = U256::from_u128(&env, current_timestamp.into());

//         immutables.timelocks =
//             TimelocksLib::set_deployed_at(&env, immutables.timelocks, current_time);

//         let rescue_delay = env
//             .storage()
//             .instance()
//             .get::<_, U256>(&RESCUE_DELAY_SRC)
//             .unwrap();

//             let mut constructor_args: Vec<Val> = Vec::new(&env);
//             constructor_args.push_back(immutables.clone().into_val(&env));
//             constructor_args.push_back(rescue_delay.into_val(&env));

//         let escrow_address = env
//             .deployer()
//             .with_address(env.current_contract_address(), salt)
//             .deploy_v2(wasm_hash, constructor_args);

//         // Transfer tokens (amount + safety deposit)
//         let total_amount = immutables.amount.add(&immutables.safety_deposit);
//         let token_client = TokenClient::new(&env, &immutables.token);
//         token_client.transfer(
//             &immutables.maker,
//             &escrow_address,
//             &(total_amount.to_u128().unwrap() as i128),
//         );

//         env.events().publish(
//             (
//                 "src_escrow_created",
//                 escrow_address.clone(),
//                 immutables.hashlock,
//                 immutables.maker,
//                 immutables.taker,
//             ),
//             (),
//         );

//         escrow_address
//     }

//     pub fn create_dst_escrow(
//         env: Env,
//         wasm_hash: BytesN<32>,
//         salt: BytesN<32>,
//         mut immutables: Immutables,
//         src_cancellation_timestamp: U256,
//     ) -> Address {
//         let current_timestamp = env.ledger().timestamp();
//         let current_time = U256::from_u128(&env, current_timestamp.into());

//         immutables.timelocks =
//             TimelocksLib::set_deployed_at(&env, immutables.timelocks, current_time);

//         let dst_cancellation = TimelocksLib::get(
//             &env,
//             immutables.timelocks.clone(),
//             Stage::DstCancellation as u32,
//         );

//         if dst_cancellation > src_cancellation_timestamp {
//             panic!("Invalid creation time");
//         }

//         let rescue_delay = env
//             .storage()
//             .instance()
//             .get::<_, U256>(&RESCUE_DELAY_DST)
//             .unwrap();

//             let mut constructor_args: Vec<Val> = Vec::new(&env);
//             constructor_args.push_back(immutables.clone().into_val(&env));
//             constructor_args.push_back(rescue_delay.into_val(&env));

//         let escrow_address = env
//             .deployer()
//             .with_address(env.current_contract_address(), salt)
//             .deploy_v2(wasm_hash, constructor_args);

//         // Transfer tokens (amount + safety deposit)
//         let total_amount = immutables.amount.add(&immutables.safety_deposit);
//         let token_client = TokenClient::new(&env, &immutables.token);
//         token_client.transfer(
//             &immutables.taker,
//             &escrow_address,
//             &(total_amount.to_u128().unwrap() as i128),
//         );

//         env.events().publish(
//             (
//                 "dst_escrow_created",
//                 escrow_address.clone(),
//                 immutables.hashlock,
//                 immutables.taker,
//             ),
//             (),
//         );

//         escrow_address
//     }
// }

// lib.rs (your main file)
#![no_std]
mod escrow;
mod timelocks;

use soroban_sdk::{
    contract, contractimpl, symbol_short, token::TokenClient, Address, Env,
    U256, Symbol,
};

pub use escrow::{Immutables};
pub use timelocks::{Stage, TimelocksLib};

// Import the deployed escrow contracts
mod escrow_src_contract {
    use crate::timelocks::Timelocks;
    soroban_sdk::contractimport!(
        file = "/home/lohith_kotni/setu-swap/stellar/target/wasm32v1-none/release/escrow_src.wasm"
    );
}

mod escrow_dst_contract {
    use crate::timelocks::Timelocks;
    soroban_sdk::contractimport!(
        file = "/home/lohith_kotni/setu-swap/stellar/target/wasm32v1-none/release/escrow_dst.wasm"
    );
}

#[contract]
pub struct EscrowFactory;

const RESCUE_DELAY_SRC: Symbol = symbol_short!("res_src");
const RESCUE_DELAY_DST: Symbol = symbol_short!("res_dst");
const ACCESS_TOKEN: Symbol = symbol_short!("acc_token");



#[contractimpl]
impl EscrowFactory {
    pub fn __constructor(env: Env, access_token: Address, rescue_delay_src: U256, rescue_delay_dst: U256) {
        env.storage().instance().set(&ACCESS_TOKEN, &access_token);
        env.storage().instance().set(&RESCUE_DELAY_SRC, &rescue_delay_src);
        env.storage().instance().set(&RESCUE_DELAY_DST, &rescue_delay_dst);
    }

    pub fn create_src_escrow(
        env: Env,
        escrow_contract_address: Address,
        mut immutables: Immutables,
    ) -> Address {
        let current_timestamp = env.ledger().timestamp();
        let current_time = U256::from_u128(&env, current_timestamp.into());

        immutables.timelocks =
            TimelocksLib::set_deployed_at(&env, immutables.timelocks, current_time);

        let rescue_delay = env
            .storage()
            .instance()
            .get::<_, U256>(&RESCUE_DELAY_SRC)
            .unwrap();

        // Create client for the existing escrow contract
        let escrow_client = escrow_src_contract::Client::new(&env, &escrow_contract_address);
        let access_token = env
    .storage()
    .instance()
    .get::<_, Address>(&ACCESS_TOKEN)
    .expect("access token not set");

        
        // Call the constructor/initialize function
        escrow_client.constructor(&access_token, &rescue_delay);

        // Transfer tokens (amount + safety deposit)
        let total_amount = immutables.amount.add(&immutables.safety_deposit);
        let token_client = TokenClient::new(&env, &immutables.token);
        token_client.transfer(
            &immutables.maker,
            &escrow_contract_address,
            &(total_amount.to_u128().unwrap() as i128),
        );

        env.events().publish(
            (
                "src_escrow_created",
                escrow_contract_address.clone(),
                immutables.hashlock,
                immutables.maker,
                immutables.taker,
            ),
            (),
        );

        escrow_contract_address
    }

    pub fn create_dst_escrow(
        env: Env,
        escrow_contract_address: Address,
        mut immutables: Immutables,
        src_cancellation_timestamp: U256,
    ) -> Address {
        let current_timestamp = env.ledger().timestamp();
        let current_time = U256::from_u128(&env, current_timestamp.into());

        immutables.timelocks =
            TimelocksLib::set_deployed_at(&env, immutables.timelocks, current_time);

        let dst_cancellation = TimelocksLib::get(
            &env,
            immutables.timelocks.clone(),
            Stage::DstCancellation as u32,
        );

        if dst_cancellation > src_cancellation_timestamp {
            panic!("Invalid creation time");
        }

        let rescue_delay = env
            .storage()
            .instance()
            .get::<_, U256>(&RESCUE_DELAY_DST)
            .unwrap();

        // Create client for the existing escrow contract
        let escrow_client = escrow_dst_contract::Client::new(&env, &escrow_contract_address);
        let access_token = env
    .storage()
    .instance()
    .get::<_, Address>(&ACCESS_TOKEN)
    .expect("access token not set");

        
        // Call the constructor/initialize function
        escrow_client.constructor(&access_token, &rescue_delay);

        // Transfer tokens (amount + safety deposit)
        let total_amount = immutables.amount.add(&immutables.safety_deposit);
        let token_client = TokenClient::new(&env, &immutables.token);
        token_client.transfer(
            &immutables.taker,
            &escrow_contract_address,
            &(total_amount.to_u128().unwrap() as i128),
        );

        env.events().publish(
            (
                "dst_escrow_created",
                escrow_contract_address.clone(),
                immutables.hashlock,
                immutables.taker,
            ),
            (),
        );

        escrow_contract_address
    }
}

