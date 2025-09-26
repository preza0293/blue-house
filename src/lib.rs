#![no_std]
mod common;
#[cfg(not(feature = "no-entrypoint"))]
mod swap_handler;
#[cfg(feature = "std")]
extern crate std;
use crate::common::*;
use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, default_panic_handler, no_allocator,
    program_entrypoint, program_error::ProgramError, pubkey::Pubkey,
};

#[derive(Clone)]
pub struct BaseAccounts {
    pub payer: AccountInfo,
    pub token_a_mint: AccountInfo,
    pub token_a_ata: AccountInfo,
    pub token_a_program: AccountInfo,
    pub system_program: AccountInfo,
    pub ata_program: AccountInfo,
    pub token_b_mint: AccountInfo,
    pub token_b_program: AccountInfo,
    pub token_b_ata: AccountInfo,
}
pub struct Bluehouse {
    base: BaseAccounts,
}
impl<'a> Bluehouse {
    pub fn new(base: BaseAccounts) -> Self {
        Self { base }
    }
}

#[repr(u8)]
pub enum ProgramInstruction {
    Swap,
}
impl TryFrom<&u8> for ProgramInstruction {
    type Error = ProgramError;
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(ProgramInstruction::Swap),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
/*
pub fn swap(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    /*
        wallet           0
        token_a_mint     1
        token_a_ata      2
        token_program    3
        sys_program      4
        ata_program      5
        token_b_mint     6
        token_b_program  7
        token_b_ata      8
        dex_id_          9
        ... swap accounts ...
    */
    let swap_data = bytemuck::try_from_bytes::<SwapData>(data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let amount_in = swap_data.amount_in;
    let amount_out = swap_data.amount_out;
    let a_to_b = swap_data.a_to_b != 0;
    let program_id = *accounts[9].key();
    if !KNOWN_PROGRAMS.contains(&program_id) {
        return Err(ProgramError::InvalidInstructionData);
    }
    match program_id {
        x if x == DLMM_PROGRAM_ID => process_meteora_dlmm_swap(accounts, amount_in, a_to_b, 9)?,
        x if x == DAMM_PROGRAM_ID => process_meteora_damm_swap(accounts, amount_in, a_to_b, 9)?,
        x if x == WHIRLPOOLS_PROGRAM_ID => process_orca_swap_v2(accounts, amount_in, a_to_b, 9)?,
        x if x == RAY_AMM_PROGRAM_ID => process_ray_amm_swap(accounts, amount_in, a_to_b, 9)?,
        x if x == RAY_CPMM_PROGRAM_ID => process_ray_cpmm_swap(accounts, amount_in, a_to_b, 9)?,
        x if x == RAY_CL_PROGRAM_ID => process_ray_cl_swap(accounts, amount_in, a_to_b, 9)?,
        x if x == HUMIDIFI_PROGRAM_ID => process_humidifi_swap(accounts, amount_in, a_to_b, 9)?,
        x if x == PUMP_AMM_PROGRAM_ID && u64::from_le_bytes(amount_out) != 0 => {
            process_pump_buy(accounts, amount_in, amount_out, 9)?;
        }
        x if x == PUMP_AMM_PROGRAM_ID => {
            process_pump_sell(accounts, amount_in, 9)?;
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SwapData {
    pub amount_in: [u8; 8],
    pub amount_out: [u8; 8], //only for pump buy ,else set to 0
    pub a_to_b: u8,
}
*/
