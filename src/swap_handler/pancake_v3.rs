use crate::Bluehouse;
use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const PANCAKE_SWAP_FLAGS: &[u8] = &[2, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]; //11-13
/*
    pub program_id: AccountInfo,
    pub pool: AccountInfo,
    pub amm_config: AccountInfo,
    pub observation: AccountInfo,
    pub bitmap_extension: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
    pub tick_array_0: AccountInfo,
    pub tick_array_1: AccountInfo,
    pub tick_array_2: AccountInfo,
*/
pub fn process_pancake_v3_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
    is_base_input: Option<bool>,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);

    let (input_vault, output_vault) = if a_to_b {
        (&accounts[offset + 5], &accounts[offset + 6])
    } else {
        (&accounts[offset + 6], &accounts[offset + 5])
    };

    let mut cpi_accounts: ArrayVec<&AccountInfo, 13> = ArrayVec::new();
    cpi_accounts.extend([
        &accounts[offset + 0],    // program_id / payer
        &accounts[offset + 2],    // amm_config
        &accounts[offset + 1],    // pool
        token_in_ata,             // input ata
        token_out_ata,            // output ata
        input_vault,              // input vault
        output_vault,             // output vault
        &accounts[offset + 3],    // observation
        &bh.base.token_a_program, // token program
        &accounts[offset + 7],    // tick_array_0
        &accounts[offset + 4],    // bitmap_extension
    ]);

    // Conditionally push optional tick arrays
    if accounts[offset + 8].lamports() != 0 {
        cpi_accounts.push(&accounts[offset + 8]);
    }
    if accounts[offset + 9].lamports() != 0 {
        cpi_accounts.push(&accounts[offset + 9]);
    }

    let sqrt_price_limit_x64 = 0u128;
    let other_amount_threshold = 1u64;
    let mut instr_data = [0u8; 41];
    instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount); // amount
    instr_data[16..24].copy_from_slice(&other_amount_threshold.to_le_bytes());
    instr_data[24..40].copy_from_slice(&sqrt_price_limit_x64.to_le_bytes());
    instr_data[40] = is_base_input.unwrap_or(true) as u8;

    execute_cpi::<13>(
        &PANCAKE_SWAP_V3_PROGRAM_ID,
        &cpi_accounts,
        &PANCAKE_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
