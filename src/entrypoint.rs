#![allow(unexpected_cfgs)]
use crate::common::*;
use crate::swap_handler::humidifi::process_humidifi_swap;
use crate::swap_handler::pump::{process_pump_buy, process_pump_sell};
use crate::swap_handler::ray_clmm::process_ray_cl_swap;
use crate::swap_handler::ray_cpmm::process_ray_cpmm_swap;
use crate::swap_handler::{
    meteora_damm_v2::process_meteora_damm_swap, meteora_dlmm::process_meteora_dlmm_swap,
    ray_amm::process_ray_amm_swap, whirlpool::process_orca_swap_v2,
};
use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, default_panic_handler, no_allocator,
    program_entrypoint, program_error::ProgramError, pubkey::Pubkey,
};
program_entrypoint!(process_instruction);
no_allocator!();
default_panic_handler!();
#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let ix = ProgramInstruction::try_from(ix_disc)?;
    match ix {
        ProgramInstruction::Swap => swap(accounts, &instruction_data)?,
    }
    Ok(())
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
