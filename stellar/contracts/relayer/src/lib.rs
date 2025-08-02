#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contractclient, Address, Bytes, BytesN, Env, Vec,
    contracterror, panic_with_error, symbol_short
};

/// Error codes
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum RelayerError {
    Unauthorized = 1,
}

/// Storage keys
#[contracttype]
pub enum DataKey {
    AuthorizedRelayers(Address), // admin -> Vec<Address>
}

/// Events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretSharedEvent {
    pub order_hash: BytesN<32>,
    pub part_idx: u32,
}

/// External escrow contract client
#[contractclient(name = "EscrowContractClient")]
pub trait EscrowContractTrait {
    fn escrow_exists(local_escrow_addr: Address) -> bool;
    fn get_escrow_info(local_escrow_addr: Address) -> (Address, Address, i128, BytesN<32>, Vec<u64>, bool, bool, bool, BytesN<32>);
    fn compute_order_hash_from_info(
        maker: Address,
        taker: Address,
        token_type: Address,
        amount: i128,
        hashlock: BytesN<32>,
        timelocks: Vec<u64>,
        is_src: bool,
    ) -> BytesN<32>;
    fn get_escrow_token_type(local_escrow_addr: Address) -> Address;
    fn get_escrow_safety_deposit(local_escrow_addr: Address) -> i128;
}

#[contract]
pub struct SecretRelayerContract;

#[contractimpl]
impl SecretRelayerContract {
    
    /// Initialize relayer authority - only callable once by deployer
    pub fn initialize(env: Env, account: Address) {
        account.require_auth();
        
        let relayers: Vec<Address> = Vec::new(&env);
        env.storage().persistent().set(&DataKey::AuthorizedRelayers(account), &relayers);
    }

    /// Add authorized relayer - only callable by deployer
    pub fn add_relayer(env: Env, admin: Address, relayer: Address) {
        admin.require_auth();
        
        let key = DataKey::AuthorizedRelayers(admin.clone());
        if !env.storage().persistent().has(&key) {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }
        
        let mut authority: Vec<Address> = env.storage().persistent().get(&key).unwrap();
        authority.push_back(relayer);
        env.storage().persistent().set(&key, &authority);
    }

    /// Remove authorized relayer - only callable by deployer
    pub fn remove_relayer(env: Env, admin: Address, relayer: Address) {
        admin.require_auth();
        
        let key = DataKey::AuthorizedRelayers(admin.clone());
        if !env.storage().persistent().has(&key) {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }
        
        let mut authority: Vec<Address> = env.storage().persistent().get(&key).unwrap();
        let mut found_index: Option<u32> = None;
        
        for i in 0..authority.len() {
            if authority.get(i).unwrap() == relayer {
                found_index = Some(i);
                break;
            }
        }
        
        if let Some(index) = found_index {
            authority.remove(index);
            env.storage().persistent().set(&key, &authority);
        }
    }

    /// Emit secret shared event - only callable by authorized relayers
    pub fn emit_secret_shared(
        env: Env,
        relayer: Address,
        admin: Address,
        local_escrow_addr: Address,
        order_hash: BytesN<32>,
        part_idx: u32,
        escrow_contract: Address,
    ) {
        relayer.require_auth();
        
        let key = DataKey::AuthorizedRelayers(admin.clone());
        if !env.storage().persistent().has(&key) {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }
        
        let authority: Vec<Address> = env.storage().persistent().get(&key).unwrap();
        let mut found = false;
        for i in 0..authority.len() {
            if authority.get(i).unwrap() == relayer {
                found = true;
                break;
            }
        }
        if !found {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }

        let escrow_client = EscrowContractClient::new(&env, &escrow_contract);

        // 1. Verify local escrow exists
        if !escrow_client.escrow_exists(&local_escrow_addr) {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }

        // 2. Get local escrow info
        let (maker, taker, amount, hashlock, timelocks, claimed, cancelled, is_src, _) = 
            escrow_client.get_escrow_info(&local_escrow_addr);

        // 3. Verify escrow is not already finalized
        if claimed || cancelled {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }

        // 4. Verify local escrow has passed STAGE_FINALITY timelock
        let current_time = env.ledger().timestamp();
        let finality_deadline = timelocks.get(0).unwrap(); // STAGE_FINALITY = 0
        if current_time < finality_deadline {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }

        // 5. Verify order hash matches local escrow (prevents replay attacks)
        let token_type = escrow_client.get_escrow_token_type(&local_escrow_addr);
        let recomputed_hash = escrow_client.compute_order_hash_from_info(
            &maker,
            &taker,
            &token_type,
            &amount,
            &hashlock,
            &timelocks,
            &is_src
        );
        if recomputed_hash != order_hash {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }

        // 6. Verify safety deposit exists (incentive mechanism)
        let safety_deposit = escrow_client.get_escrow_safety_deposit(&local_escrow_addr);
        if safety_deposit <= 0 {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }

        // Emit the secret shared event
        env.events().publish(
            (symbol_short!("secret"),),
            SecretSharedEvent {
                order_hash,
                part_idx,
            },
        );
    }

    /// Check if address is authorized relayer
    pub fn is_authorized_relayer(env: Env, admin: Address, relayer: Address) -> bool {
        let key = DataKey::AuthorizedRelayers(admin);
        if !env.storage().persistent().has(&key) {
            return false;
        }
        
        let authority: Vec<Address> = env.storage().persistent().get(&key).unwrap();
        for i in 0..authority.len() {
            if authority.get(i).unwrap() == relayer {
                return true;
            }
        }
        false
    }

    /// Get all authorized relayers
    pub fn get_authorized_relayers(env: Env, admin: Address) -> Vec<Address> {
        let key = DataKey::AuthorizedRelayers(admin);
        if !env.storage().persistent().has(&key) {
            panic_with_error!(&env, RelayerError::Unauthorized);
        }
        
        env.storage().persistent().get(&key).unwrap()
    }
}