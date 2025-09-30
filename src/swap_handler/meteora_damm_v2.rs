use crate::Bluehouse;
use crate::common::*;
use pinocchio::{
    ProgramResult, account_info::AccountInfo, msg, program_error::ProgramError,
    syscalls::sol_log_pubkey,
};
const DAMM_V2_SWAP_FLAGS: &[u8] = &[0, 1, 1, 1, 1, 1, 0, 0, 2, 0, 0, 0, 0, 0]; //14
/*
    pub program_id: AccountInfo,
    pub event_auth: AccountInfo,
    pub pool_auth: AccountInfo,
    pub pool: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
*/

pub fn process_meteora_damm_v2_swap(
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

    let cpi_accounts = [
        &accounts[offset + 2],    // pool_auth
        &accounts[offset + 3],    // pool
        token_in_ata,             // user token in ATA
        token_out_ata,            // user token out ATA
        &accounts[offset + 4],    // vault A
        &accounts[offset + 5],    // vault B
        &bh.base.token_a_mint,    // mint A
        &bh.base.token_b_mint,    // mint B
        &bh.base.payer,           // payer / user
        &bh.base.token_a_program, // token A program
        &bh.base.token_b_program, // token B program
        &accounts[offset + 0],    // program_id (referrer token account)
        &accounts[offset + 1],    // event_auth
        &accounts[offset + 0],    // program ID
    ];
    execute_cpi::<14>(
        &DAMM_PROGRAM_ID,
        &cpi_accounts,
        &DAMM_V2_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
