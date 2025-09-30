use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
//args len=24
const LIFINITY_SWAP_FLAGS: &[u8] = &[0, 1, 2, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0]; //13
/*
    pub program_id: AccountInfo,
    pub pool: AccountInfo,
    pub auth: AccountInfo,
    pub pool_mint: AccountInfo,
    pub fee_account: AccountInfo,
    pub oracle_main_account: AccountInfo,
    pub oracle_sub_account: AccountInfo,
    pub oracle_pc_account: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
*/
pub fn process_lifinity_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
) -> ProgramResult {
    let mut instr_data = [0u8; 24];
    instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount); // amount
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); // flag

    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);

    let (swap_source, swap_destination) = if a_to_b {
        (&accounts[offset + 8], &accounts[offset + 9])
    } else {
        (&accounts[offset + 9], &accounts[offset + 8])
    };

    let cpi_accounts = [
        &accounts[offset + 2],    // auth
        &accounts[offset + 1],    // pool
        &bh.base.payer,           // user transfer auth
        token_in_ata,             // source
        token_out_ata,            // destination
        swap_source,              // swap source
        swap_destination,         // swap destination
        &accounts[offset + 3],    // pool mint
        &accounts[offset + 4],    // fee account
        &bh.base.token_a_program, // token program
        &accounts[offset + 5],    // oracle main account
        &accounts[offset + 6],    // oracle sub account
        &accounts[offset + 7],    // oracle pc account
    ];

    execute_cpi::<13>(
        &LIFINITY_PROGRAM_ID,
        &cpi_accounts,
        &LIFINITY_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
