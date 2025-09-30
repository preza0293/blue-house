#![allow(unexpected_cfgs)]
mod common;
#[cfg(not(feature = "no-entrypoint"))]
mod swap_handler;
use crate::{
    common::*,
    swap_handler::{
        lifinity::process_lifinity_swap,
        meteora_damm_v2::*,
        meteora_dlmm::*,
        obric_v2::process_obric_v2_swap,
        pancake_v3::process_pancake_v3_swap,
        phoniex::process_phoniex_swap,
        pump::*,
        ray_amm::*,
        ray_clmm::*,
        ray_cpmm::*,
        saros::{process_saros_dlmm_swap, process_saros_swap},
        solfi::process_solfi_swap,
        stabble::process_stabble_swap,
        vertigo::{process_vertigo_buy, process_vertigo_sell},
        whirlpool::*,
    },
};

use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, default_panic_handler, no_allocator,
    program_entrypoint, program_error::ProgramError, pubkey::Pubkey,
};
program_entrypoint!(process_instruction);
no_allocator!();
default_panic_handler!();
pinocchio_pubkey::declare_id!("ENrRns55VechXJiq4bMbdx7idzabctvaEJoYeWxRNe7Y");
#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    swap(accounts, &instruction_data)?;
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
    pub base: BaseAccounts,
}
impl Bluehouse {
    pub fn from_slice(slice: &[AccountInfo]) -> Result<Self, ProgramError> {
        if slice.len() < 9 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        Ok(Self {
            base: BaseAccounts {
                payer: slice[0].clone(),
                token_a_mint: slice[1].clone(),
                token_a_ata: slice[2].clone(),
                token_a_program: slice[3].clone(),
                system_program: slice[4].clone(),
                ata_program: slice[5].clone(),
                token_b_mint: slice[6].clone(),
                token_b_program: slice[7].clone(),
                token_b_ata: slice[8].clone(),
            },
        })
    }
}
//tthe program currenlty supports one ix ->swap
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
    let bh = Bluehouse::from_slice(&accounts[0..9])?;

    let amount_in: [u8; 8] = data[1..9].try_into().unwrap();
    let amount_out: [u8; 8] = data[9..17].try_into().unwrap();
    let a_to_b: bool = data[17] != 0;

    let program_id = *accounts[9].key();
    if !KNOWN_PROGRAMS.contains(&program_id) {
        return Err(ProgramError::InvalidInstructionData);
    }

    match program_id {
        x if x == DLMM_PROGRAM_ID => {
            process_meteora_dlmm_swap(&bh, accounts, 9, amount_in, a_to_b)?
        }
        x if x == DAMM_PROGRAM_ID => {
            process_meteora_damm_v2_swap(&bh, accounts, 9, amount_in, a_to_b)?
        }
        x if x == WHIRLPOOLS_PROGRAM_ID => {
            process_orca_swap_v2(&bh, accounts, 9, amount_in, a_to_b, None)?
        }
        x if x == RAY_AMM_PROGRAM_ID => process_ray_amm_swap(&bh, accounts, 9, amount_in, a_to_b)?,
        x if x == RAY_CPMM_PROGRAM_ID => {
            process_ray_cpmm_swap(&bh, accounts, 9, amount_in, a_to_b)?
        }
        x if x == RAY_CL_PROGRAM_ID => {
            process_ray_cl_swap(&bh, accounts, 9, amount_in, a_to_b, None)?
        }
        x if x == PUMP_AMM_PROGRAM_ID && u64::from_le_bytes(amount_out) != 0 => {
            process_pump_buy(&bh, accounts, 9, amount_in, amount_out)?;
        }
        x if x == PUMP_AMM_PROGRAM_ID => {
            process_pump_sell(&bh, accounts, 9, amount_in)?;
        }
        x if x == LIFINITY_PROGRAM_ID => {
            process_lifinity_swap(&bh, accounts, 9, amount_in, a_to_b)?;
        }
        x if x == OBRIC_V2_PROGRAM_ID => {
            process_obric_v2_swap(&bh, accounts, 9, amount_in, a_to_b)?;
        }
        x if x == PANCAKE_SWAP_V3_PROGRAM_ID => {
            process_pancake_v3_swap(&bh, accounts, 9, amount_in, a_to_b, None)?;
        }
        x if x == PHONIEX_PROGRAM_ID => {
            process_phoniex_swap(&bh, accounts, 9, amount_in, a_to_b)?;
        }
        x if x == SAROS_PROGRAM_ID => {
            process_saros_swap(&bh, accounts, 9, amount_in, a_to_b)?;
        }
        x if x == SAROS_DLMM_PROGRAM_ID => {
            process_saros_dlmm_swap(&bh, accounts, 9, amount_in, a_to_b, None)?;
        }
        x if x == SOLFI_PROGRAM_ID => {
            process_solfi_swap(&bh, accounts, 9, amount_in, a_to_b)?;
        }
        x if x == STABBLE_PROGRAM_ID => {
            process_stabble_swap(&bh, accounts, 9, amount_in, a_to_b)?;
        }
        x if x == VERTIGO_PROGRAM_ID && u64::from_le_bytes(amount_out) != 0 => {
            process_vertigo_buy(&bh, accounts, 9, amount_in, amount_out)?;
        }
        x if x == VERTIGO_PROGRAM_ID => {
            process_vertigo_sell(&bh, accounts, 9, amount_in)?;
        }

        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SwapData {
    pub amount_in: [u8; 8],
    pub amount_out: [u8; 8], //only for pump buy/vertigo buy ,else set to 0 (for pump & vertigo it shoud be >0)
    pub a_to_b: u8,
}
