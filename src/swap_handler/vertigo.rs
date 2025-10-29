use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const VERTIGO_SWAP_FLAGS: &[u8] = &[1, 2, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]; //13
/*
  pub program_id: AccountInfo,
   pub pool: AccountInfo,
   pub pool_owner: AccountInfo,
   pub vault_a: AccountInfo,
   pub vault_b: AccountInfo,
*/
pub fn process_vertigo_buy(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    min_amount_out: [u8; 8],
) -> ProgramResult {
    let cpi_accounts: [&AccountInfo; 13] = [
        &accounts[offset + 1],   // pool
        bh.base.payer,           // user
        &accounts[offset + 2],   // pool_owner
        bh.base.token_a_mint,    // mint a
        bh.base.token_b_mint,    // mint b
        bh.base.token_a_ata,     // user ata a
        bh.base.token_b_ata,     // user ata b
        &accounts[offset + 3],   // vault a
        &accounts[offset + 4],   // vault b
        bh.base.token_a_program, // token program a
        bh.base.token_b_program, // token program b
        bh.base.system_program,  // sys program
        &accounts[offset],       // program id
    ];

    let mut instr_data = [0u8; 24];
    instr_data[0..8].copy_from_slice(VERTIGO_BUY_SELECTOR);
    instr_data[8..16].copy_from_slice(&amount);
    instr_data[16..24].copy_from_slice(&min_amount_out);

    execute_cpi::<13>(
        &VERTIGO_PROGRAM_ID,
        &cpi_accounts,
        VERTIGO_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
pub fn process_vertigo_sell(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
) -> ProgramResult {
    let cpi_accounts: [&AccountInfo; 13] = [
        &accounts[offset + 1],   // pool
        bh.base.payer,           // user
        &accounts[offset + 2],   // pool_owner
        bh.base.token_a_mint,    // mint a
        bh.base.token_b_mint,    // mint b
        bh.base.token_a_ata,     // user ata a
        bh.base.token_b_ata,     // user ata b
        &accounts[offset + 3],   // vault a
        &accounts[offset + 4],   // vault b
        bh.base.token_a_program, // token program a
        bh.base.token_b_program, // token program b
        bh.base.system_program,  // sys program
        &accounts[offset],       // program id
    ];

    let mut instr_data = [0u8; 24];
    instr_data[0..8].copy_from_slice(VERTIGO_SELL_SELECTOR);
    instr_data[8..16].copy_from_slice(&amount);
    instr_data[16..24].copy_from_slice(&0u64.to_le_bytes());

    execute_cpi::<13>(
        &VERTIGO_PROGRAM_ID,
        &cpi_accounts,
        VERTIGO_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
