//#![no_std]
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

use pinocchio::{
    ProgramResult, account_info::AccountInfo, default_panic_handler, no_allocator,
    program_entrypoint, program_error::ProgramError, pubkey::Pubkey,
};
const SWAP_DATA_SIZE: usize = 17;
program_entrypoint!(process_instruction);
//no_allocator!();
default_panic_handler!();
pinocchio_pubkey::declare_id!("b1225DYKeTyLGd4SrNZFPcgzCC76Q9qniud7XgwB7C4");
#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    swap(accounts, &instruction_data)?;
    Ok(())
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SwapData {
    pub amount_in: [u8; 8],
    pub amount_out: [u8; 8], //for pump & vertigo it shoud be >0 ,else set to 0
    pub a_to_b: bool,
}
impl SwapData {
    pub fn from_bytes(data: &[u8]) -> &Self {
        assert_eq!(data.len(), 17);
        unsafe { &*(data.as_ptr() as *const SwapData) }
    }
}

pub struct BaseAccounts<'a> {
    pub payer: &'a AccountInfo,
    pub token_a_mint: &'a AccountInfo,
    pub token_a_ata: &'a AccountInfo,
    pub token_a_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub ata_program: &'a AccountInfo,
    pub token_b_mint: &'a AccountInfo,
    pub token_b_program: &'a AccountInfo,
    pub token_b_ata: &'a AccountInfo,
}

pub struct Bluehouse<'a> {
    pub base: BaseAccounts<'a>,
}

impl<'a> Bluehouse<'a> {
    pub fn from_slice(slice: &'a [AccountInfo]) -> Result<Self, ProgramError> {
        if slice.len() < 9 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        Ok(Self {
            base: BaseAccounts {
                payer: &slice[0],
                token_a_mint: &slice[1],
                token_a_ata: &slice[2],
                token_a_program: &slice[3],
                system_program: &slice[4],
                ata_program: &slice[5],
                token_b_mint: &slice[6],
                token_b_program: &slice[7],
                token_b_ata: &slice[8],
            },
        })
    }
}
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
pub fn swap(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let bh = Bluehouse::from_slice(&accounts[0..9])?;

    if accounts.len() <= 9 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let mut account = 9usize;
    let mut swap_index = 0usize;
    while account < accounts.len() {
        let program_id = *accounts[account].key();
        let swap_data: SwapData = *SwapData::from_bytes(data);

        if KNOWN_PROGRAMS.contains(&program_id) {
            swap_index = swap_index + 1;
            match program_id {
                x if x == DLMM_PROGRAM_ID => process_meteora_dlmm_swap(
                    &bh,
                    accounts,
                    account,
                    swap_data.amount_in,
                    swap_data.a_to_b,
                )?,
                x if x == DAMM_PROGRAM_ID => process_meteora_damm_v2_swap(
                    &bh,
                    accounts,
                    account,
                    swap_data.amount_in,
                    swap_data.a_to_b,
                )?,
                x if x == WHIRLPOOLS_PROGRAM_ID => process_orca_swap_v2(
                    &bh,
                    accounts,
                    account,
                    swap_data.amount_in,
                    swap_data.a_to_b,
                    None,
                )?,
                x if x == RAY_AMM_PROGRAM_ID => process_ray_amm_swap(
                    &bh,
                    accounts,
                    account,
                    swap_data.amount_in,
                    swap_data.a_to_b,
                )?,
                x if x == RAY_CPMM_PROGRAM_ID => process_ray_cpmm_swap(
                    &bh,
                    accounts,
                    account,
                    swap_data.amount_in,
                    swap_data.a_to_b,
                )?,
                x if x == RAY_CL_PROGRAM_ID => process_ray_cl_swap(
                    &bh,
                    accounts,
                    account,
                    swap_data.amount_in,
                    swap_data.a_to_b,
                    None,
                )?,
                x if x == PUMP_AMM_PROGRAM_ID && u64::from_le_bytes(swap_data.amount_out) != 0 => {
                    process_pump_buy(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.amount_out,
                    )?;
                }
                x if x == PUMP_AMM_PROGRAM_ID => {
                    process_pump_sell(&bh, accounts, account, swap_data.amount_in)?;
                }
                x if x == LIFINITY_PROGRAM_ID => {
                    process_lifinity_swap(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.a_to_b,
                    )?;
                }
                x if x == OBRIC_V2_PROGRAM_ID => {
                    process_obric_v2_swap(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.a_to_b,
                    )?;
                }
                x if x == PANCAKE_SWAP_V3_PROGRAM_ID => {
                    process_pancake_v3_swap(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.a_to_b,
                        None,
                    )?;
                }
                x if x == PHONIEX_PROGRAM_ID => {
                    process_phoniex_swap(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.a_to_b,
                    )?;
                }
                x if x == SAROS_PROGRAM_ID => {
                    process_saros_swap(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.a_to_b,
                    )?;
                }
                x if x == SAROS_DLMM_PROGRAM_ID => {
                    process_saros_dlmm_swap(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.a_to_b,
                        None,
                    )?;
                }
                x if x == SOLFI_PROGRAM_ID => {
                    process_solfi_swap(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.a_to_b,
                    )?;
                }
                x if x == STABBLE_PROGRAM_ID => {
                    process_stabble_swap(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.a_to_b,
                    )?;
                }
                x if x == VERTIGO_PROGRAM_ID && u64::from_le_bytes(swap_data.amount_out) != 0 => {
                    process_vertigo_buy(
                        &bh,
                        accounts,
                        account,
                        swap_data.amount_in,
                        swap_data.amount_out,
                    )?;
                }
                x if x == VERTIGO_PROGRAM_ID => {
                    process_vertigo_sell(&bh, accounts, account, swap_data.amount_in)?;
                }
                _ => return Err(ProgramError::InvalidInstructionData),
            }
        }

        account = account + 1;
    }

    Ok(())
}
