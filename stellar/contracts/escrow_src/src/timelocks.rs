use soroban_sdk::{Env, U256};
pub type Timelocks = U256;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Stage {
    SrcWithdrawal = 0,
    SrcPublicWithdrawal = 1,
    SrcCancellation = 2,
    SrcPublicCancellation = 3,
    DstWithdrawal = 4,
    DstPublicWithdrawal = 5,
    DstCancellation = 6,
}

const DEPLOYED_AT_OFFSET: u32 = 224;

pub struct TimelocksLib;

impl TimelocksLib {
    pub fn set_deployed_at(_env: &Env, timelocks: Timelocks, value: U256) -> Timelocks {
        let cleared = timelocks.shl(32).shr(32);
        let shifted_value = value.shl(DEPLOYED_AT_OFFSET);
        cleared.add(&shifted_value)
    }

    pub fn rescue_start(timelocks: Timelocks, rescue_delay: U256) -> U256 {
        let deployed_at = timelocks.shr(DEPLOYED_AT_OFFSET);
        rescue_delay.add(&deployed_at)
    }

    pub fn get(env: &Env, timelocks: Timelocks, stage: u32) -> U256 {
        let deployed_at = timelocks.shr(DEPLOYED_AT_OFFSET);
        let bit_shift = stage * 32;
        let shifted = timelocks.shr(bit_shift);
        let stage_value_u32 = shifted.to_u128().unwrap_or(0) as u32;
        let stage_value = U256::from_u32(env, stage_value_u32);
        deployed_at.add(&stage_value)
    }
}