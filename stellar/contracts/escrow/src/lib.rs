#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Bytes, BytesN, Env, Vec,
    symbol_short,
};
use soroban_sdk::xdr::ToXdr;

// Import crypto utilities
use stellar_contract_utils::crypto::keccak::Keccak256;
use stellar_contract_utils::crypto::merkle::Verifier;

// Stage constants
const STAGE_FINALITY: u32 = 0;
const STAGE_SRC_WITHDRAWAL: u32 = 1;
const STAGE_SRC_PUBLIC_WITHDRAWAL: u32 = 2;
const STAGE_SRC_CANCELLATION: u32 = 3;
const STAGE_SRC_PUBLIC_CANCELLATION: u32 = 4;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderHash {
    pub value: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashLock {
    pub value: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Escrow {
    pub maker: Address,
    pub taker: Address,
    pub token: Address,
    pub amount: i128,
    pub safety_deposit: i128,
    pub hashlock: HashLock,
    pub timelocks: Vec<u64>,
    pub claimed: bool,
    pub cancelled: bool,
    pub is_src: bool,
    pub merkle_root: BytesN<32>,
    pub parts: u32,
    pub used_parts: Vec<bool>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Escrow(BytesN<32>), 
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EscrowCreatedEvent {
    pub order_hash: BytesN<32>,
    pub maker: Address,
    pub taker: Address,
    pub token: Address,
    pub amount: i128,
    pub hashlock: BytesN<32>,
    pub timelocks: Vec<u64>,
    pub is_src: bool,
    pub merkle_root: BytesN<32>,
    pub parts: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FundsClaimedEvent {
    pub order_hash: BytesN<32>,
    pub preimage: Bytes,
    pub recipient: Address,
    pub amount: i128,
    pub merkle_root: BytesN<32>,
    pub parts: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FundsRefundedEvent {
    pub order_hash: BytesN<32>,
    pub sender: Address,
    pub amount: i128,
    pub merkle_root: BytesN<32>,
    pub parts: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafetyDepositClaimedEvent {
    pub order_hash: BytesN<32>,
    pub executor: Address,
    pub amount: i128,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Create new escrow
    pub fn create_escrow(
        env: Env,
        maker: Address,
        taker: Address,
        token: Address,
        amount: i128,
        safety_deposit: i128,
        hashlock: BytesN<32>,
        timelocks: Vec<u64>,
        is_src: bool,
        merkle_root: BytesN<32>,
        parts: u32,
    ) -> BytesN<32> {
        // Validate inputs
        if amount <= 0 {
            panic!("Invalid amount");
        }
        if safety_deposit <= 0 {
            panic!("Invalid safety deposit");
        }
        if timelocks.len() != 5 {
            panic!("Invalid timelocks");
        }

        maker.require_auth();

        // Compute order hash for deterministic identification
        let order_hash = Self::compute_order_hash(
            &env,
            maker.clone(),
            taker.clone(),
            token.clone(),
            amount,
            hashlock.clone(),
            timelocks.clone(),
            is_src,
        );

        // Check if escrow already exists
        let escrow_key = DataKey::Escrow(order_hash.clone());
        if env.storage().persistent().has(&escrow_key) {
            panic!("Escrow already exists");
        }

        // Initialize used_parts bitmap
        let mut used_parts = Vec::new(&env);
        for _ in 0..parts {
            used_parts.push_back(false);
        }

        // Transfer tokens to contract
        let contract_address = env.current_contract_address();
        let total_amount = amount + safety_deposit;
        
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&maker, &contract_address, &total_amount);

        // Create escrow
        let escrow = Escrow {
            maker: maker.clone(),
            taker: taker.clone(),
            token: token.clone(),
            amount,
            safety_deposit,
            hashlock: HashLock { value: hashlock.clone() },
            timelocks: timelocks.clone(),
            claimed: false,
            cancelled: false,
            is_src,
            merkle_root: merkle_root.clone(),
            parts,
            used_parts,
        };

        // Store escrow
        env.storage().persistent().set(&escrow_key, &escrow);

        // Emit event
        env.events().publish(
            (symbol_short!("created"),),
            EscrowCreatedEvent {
                order_hash: order_hash.clone(),
                maker,
                taker,
                token,
                amount,
                hashlock,
                timelocks,
                is_src,
                merkle_root,
                parts,
            },
        );

        order_hash
    }

    /// Withdraw funds with preimage (private)
    pub fn withdraw(
        env: Env,
        order_hash: BytesN<32>,
        preimage: Bytes,
        merkle_proof: Vec<BytesN<32>>,
        fill_index: u32,
    ) {
        let escrow_key = DataKey::Escrow(order_hash.clone());
        
        if !env.storage().persistent().has(&escrow_key) {
            panic!("Escrow does not exist");
        }

        let mut escrow: Escrow = env.storage().persistent().get(&escrow_key).unwrap();
        
        if escrow.claimed || escrow.cancelled {
            panic!("Escrow already finalized");
        }

        // Verify preimage - using keccak256 as per original contract
        let computed_hash = env.crypto().keccak256(&preimage).to_bytes();
        if computed_hash != escrow.hashlock.value {
            panic!("Invalid preimage");
        }

        // Verify partial fill if using Merkle tree
        if escrow.parts > 1 {
            if fill_index >= escrow.parts {
                panic!("Invalid fill index");
            }
            
            let used = escrow.used_parts.get(fill_index).unwrap_or(false);
            if used {
                panic!("Part already used");
            }

            // Verify Merkle proof using the proper crypto library
            // Note: Using Keccak256 for Merkle tree as it's more standard for blockchain
            let leaf = env.crypto().keccak256(&preimage).to_bytes();
            if !Verifier::<Keccak256>::verify_with_index(
                &env, 
                merkle_proof, 
                escrow.merkle_root.clone(), 
                leaf, 
                fill_index
            ) {
                panic!("Invalid Merkle proof");
            }

            // Mark part as used
            escrow.used_parts.set(fill_index, true);
        }

        // Check authorization and timing
        let current_time = env.ledger().timestamp();
        let withdrawal_time = escrow.timelocks.get(STAGE_SRC_WITHDRAWAL).unwrap();
        let cancellation_time = escrow.timelocks.get(STAGE_SRC_CANCELLATION).unwrap();

        if current_time < withdrawal_time {
            panic!("Stage not reached");
        }
        if current_time >= cancellation_time {
            panic!("Stage expired");
        }

        // Determine recipient and verify authorization
        let (recipient, caller) = if escrow.is_src {
            // EscrowSrc: only taker can withdraw, tokens go to taker
            (escrow.taker.clone(), escrow.taker.clone())
        } else {
            // EscrowDst: only taker can withdraw, tokens go to maker
            (escrow.maker.clone(), escrow.taker.clone())
        };

        caller.require_auth();

        // Mark as claimed
        escrow.claimed = true;
        env.storage().persistent().set(&escrow_key, &escrow);

        // Transfer funds
        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &escrow.token);
        
        // Transfer amount to recipient
        token_client.transfer(&contract_address, &recipient, &escrow.amount);

        // Return safety deposit to maker (resolver)
        if escrow.safety_deposit > 0 {
            token_client.transfer(&contract_address, &escrow.maker, &escrow.safety_deposit);

            env.events().publish(
                (symbol_short!("safety"),),
                SafetyDepositClaimedEvent {
                    order_hash: order_hash.clone(),
                    executor: escrow.maker.clone(),
                    amount: escrow.safety_deposit,
                },
            );
        }

        // Emit event
        env.events().publish(
            (symbol_short!("claimed"),),
            FundsClaimedEvent {
                order_hash,
                preimage,
                recipient,
                amount: escrow.amount,
                merkle_root: escrow.merkle_root,
                parts: escrow.parts,
            },
        );
    }

    /// Cancel escrow (private)
    pub fn cancel(env: Env, order_hash: BytesN<32>) {
        let escrow_key = DataKey::Escrow(order_hash.clone());
        
        if !env.storage().persistent().has(&escrow_key) {
            panic!("Escrow does not exist");
        }

        let mut escrow: Escrow = env.storage().persistent().get(&escrow_key).unwrap();
        
        if escrow.claimed || escrow.cancelled {
            panic!("Escrow already finalized");
        }

        // Check timing
        let current_time = env.ledger().timestamp();
        let cancellation_time = escrow.timelocks.get(STAGE_SRC_CANCELLATION).unwrap();

        if current_time < cancellation_time {
            panic!("Stage not reached");
        }

        // Determine refund recipient and verify authorization
        let (refund_recipient, caller) = if escrow.is_src {
            // EscrowSrc: only taker can cancel, tokens go back to maker
            (escrow.maker.clone(), escrow.taker.clone())
        } else {
            // EscrowDst: only taker can cancel, tokens go back to taker
            (escrow.taker.clone(), escrow.taker.clone())
        };

        caller.require_auth();

        // Mark as cancelled
        escrow.cancelled = true;
        env.storage().persistent().set(&escrow_key, &escrow);

        // Transfer funds
        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &escrow.token);
        
        // Refund amount
        token_client.transfer(&contract_address, &refund_recipient, &escrow.amount);

        // Return safety deposit to maker (resolver)
        if escrow.safety_deposit > 0 {
            token_client.transfer(&contract_address, &escrow.maker, &escrow.safety_deposit);
        }

        // Emit event
        env.events().publish(
            (symbol_short!("refunded"),),
            FundsRefundedEvent {
                order_hash,
                sender: refund_recipient,
                amount: escrow.amount,
                merkle_root: BytesN::from_array(&env, &[0u8; 32]),
                parts: 0,
            },
        );
    }

    /// Public withdraw with preimage (for relayer)
    pub fn public_withdraw(
        env: Env,
        order_hash: BytesN<32>,
        preimage: Bytes,
        merkle_proof: Vec<BytesN<32>>,
        fill_index: u32,
        executor: Address,
    ) {
        executor.require_auth();

        let escrow_key = DataKey::Escrow(order_hash.clone());
        
        if !env.storage().persistent().has(&escrow_key) {
            panic!("Escrow does not exist");
        }

        let mut escrow: Escrow = env.storage().persistent().get(&escrow_key).unwrap();
        
        if escrow.claimed || escrow.cancelled {
            panic!("Escrow already finalized");
        }

        // Verify preimage
        let computed_hash = env.crypto().keccak256(&preimage).to_bytes();
        if computed_hash != escrow.hashlock.value {
            panic!("Invalid preimage");
        }

        // Verify partial fill if using Merkle tree
        if escrow.parts > 1 {
            if fill_index >= escrow.parts {
                panic!("Invalid fill index");
            }
            
            let used = escrow.used_parts.get(fill_index).unwrap_or(false);
            if used {
                panic!("Part already used");
            }

            // Verify Merkle proof using the proper crypto library
            let leaf = env.crypto().keccak256(&preimage).to_bytes();
            if !Verifier::<Keccak256>::verify_with_index(
                &env, 
                merkle_proof, 
                escrow.merkle_root.clone(), 
                leaf, 
                fill_index
            ) {
                panic!("Invalid Merkle proof");
            }

            // Mark part as used
            escrow.used_parts.set(fill_index, true);
        }

        // Check timing for public withdrawal
        let current_time = env.ledger().timestamp();
        let public_withdrawal_time = escrow.timelocks.get(STAGE_SRC_PUBLIC_WITHDRAWAL).unwrap();
        let public_cancellation_time = escrow.timelocks.get(STAGE_SRC_PUBLIC_CANCELLATION).unwrap();

        if current_time < public_withdrawal_time {
            panic!("Stage not reached");
        }
        if current_time >= public_cancellation_time {
            panic!("Stage expired");
        }

        // Determine recipient
        let recipient = if escrow.is_src { 
            escrow.taker.clone() 
        } else { 
            escrow.maker.clone() 
        };

        // Mark as claimed
        escrow.claimed = true;
        env.storage().persistent().set(&escrow_key, &escrow);

        // Transfer funds
        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &escrow.token);
        
        // Transfer amount to recipient
        token_client.transfer(&contract_address, &recipient, &escrow.amount);

        // Award safety deposit to executor as incentive
        if escrow.safety_deposit > 0 {
            token_client.transfer(&contract_address, &executor, &escrow.safety_deposit);

            env.events().publish(
                (symbol_short!("safety"),),
                SafetyDepositClaimedEvent {
                    order_hash: order_hash.clone(),
                    executor: executor.clone(),
                    amount: escrow.safety_deposit,
                },
            );
        }

        // Emit event
        env.events().publish(
            (symbol_short!("claimed"),),
            FundsClaimedEvent {
                order_hash,
                preimage,
                recipient,
                amount: escrow.amount,
                merkle_root: BytesN::from_array(&env, &[0u8; 32]),
                parts: 0,
            },
        );
    }

    /// Public cancel (for relayer) - only available on SRC chain
    pub fn public_cancel(env: Env, order_hash: BytesN<32>, executor: Address) {
        executor.require_auth();

        let escrow_key = DataKey::Escrow(order_hash.clone());
        
        if !env.storage().persistent().has(&escrow_key) {
            panic!("Escrow does not exist");
        }

        let escrow: Escrow = env.storage().persistent().get(&escrow_key).unwrap();
        
        if escrow.claimed || escrow.cancelled {
            panic!("Escrow already finalized");
        }

        // Only available on SRC chain
        if !escrow.is_src {
            panic!("Unauthorized");
        }

        // Check timing for public cancellation
        let current_time = env.ledger().timestamp();
        let public_cancellation_time = escrow.timelocks.get(STAGE_SRC_PUBLIC_CANCELLATION).unwrap();

        if current_time < public_cancellation_time {
            panic!("Stage not reached");
        }

        // Remove escrow from storage
        env.storage().persistent().remove(&escrow_key);

        // On SRC chain: refund to maker, safety deposit to executor
        let refund_recipient = escrow.maker.clone();

        // Transfer funds
        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &escrow.token);
        
        // Refund amount to maker
        token_client.transfer(&contract_address, &refund_recipient, &escrow.amount);

        // Award safety deposit to executor
        if escrow.safety_deposit > 0 {
            token_client.transfer(&contract_address, &executor, &escrow.safety_deposit);

            env.events().publish(
                (symbol_short!("safety"),),
                SafetyDepositClaimedEvent {
                    order_hash: order_hash.clone(),
                    executor: executor.clone(),
                    amount: escrow.safety_deposit,
                },
            );
        }

        // Emit event
        env.events().publish(
            (symbol_short!("refunded"),),
            FundsRefundedEvent {
                order_hash,
                sender: refund_recipient,
                amount: escrow.amount,
                merkle_root: escrow.merkle_root,
                parts: escrow.parts,
            },
        );
    }

    /// Get escrow information
    pub fn get_escrow_info(env: Env, order_hash: BytesN<32>) -> Option<Escrow> {
        let escrow_key = DataKey::Escrow(order_hash);
        env.storage().persistent().get(&escrow_key)
    }

    /// Check if escrow exists
    pub fn escrow_exists(env: Env, order_hash: BytesN<32>) -> bool {
        let escrow_key = DataKey::Escrow(order_hash);
        env.storage().persistent().has(&escrow_key)
    }

    /// Compute order hash from parameters
    pub fn compute_order_hash(
        env: &Env,
        maker: Address,
        taker: Address,
        token: Address,
        amount: i128,
        hashlock: BytesN<32>,
        timelocks: Vec<u64>,
        is_src: bool,
    ) -> BytesN<32> {
        let mut data = Bytes::new(env);
        data.append(&maker.to_xdr(env));
        data.append(&taker.to_xdr(env));
        data.append(&token.to_xdr(env));
        data.append(&amount.to_xdr(env));
        data.append(&hashlock.to_xdr(env));
        data.append(&timelocks.to_xdr(env));
        data.append(&is_src.to_xdr(env));
    
        env.crypto().keccak256(&data).to_bytes()
    }

    /// Helper function to compute timelock deadline
    pub fn get_timelock_deadline(init_time: u64, stage: u32, rescue_delay: u32) -> u64 {
        match stage {
            STAGE_FINALITY => init_time,
            STAGE_SRC_WITHDRAWAL => init_time + (rescue_delay as u64),
            STAGE_SRC_PUBLIC_WITHDRAWAL => init_time + (rescue_delay as u64) * 2,
            STAGE_SRC_CANCELLATION => init_time + (rescue_delay as u64) * 3,
            STAGE_SRC_PUBLIC_CANCELLATION => init_time + (rescue_delay as u64) * 4,
            _ => panic!("Invalid stage"),
        }
    }

    pub fn get_current_stage(env: Env, order_hash: BytesN<32>) -> u32 {
        let escrow_key = DataKey::Escrow(order_hash);
        
        if !env.storage().persistent().has(&escrow_key) {
            panic!("Escrow does not exist");
        }
     
        let escrow: Escrow = env.storage().persistent().get(&escrow_key).unwrap();
        let current_time = env.ledger().timestamp();
     
        let finality_time = escrow.timelocks.get(STAGE_FINALITY).unwrap();
        let withdrawal_time = escrow.timelocks.get(STAGE_SRC_WITHDRAWAL).unwrap();
        let public_withdrawal_time = escrow.timelocks.get(STAGE_SRC_PUBLIC_WITHDRAWAL).unwrap();
        let cancellation_time = escrow.timelocks.get(STAGE_SRC_CANCELLATION).unwrap();
        let public_cancellation_time = escrow.timelocks.get(STAGE_SRC_PUBLIC_CANCELLATION).unwrap();
     
        if current_time < finality_time {
            STAGE_FINALITY
        } else if current_time < withdrawal_time {
            STAGE_FINALITY
        } else if current_time < public_withdrawal_time {
            STAGE_SRC_WITHDRAWAL
        } else if current_time < cancellation_time {
            STAGE_SRC_PUBLIC_WITHDRAWAL
        } else if current_time < public_cancellation_time {
            STAGE_SRC_CANCELLATION
        } else {
            STAGE_SRC_PUBLIC_CANCELLATION
        }
     }
}