use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const SOLFI_SWAP_FLAGS: &[u8] = &[2, 1, 1, 1, 1, 1, 0, 0]; //8
/*
accounts passed to solfi
program_id,offset+0
first_mint,offser+1
sysvar ix,offset+2
pool,offset+3
vault a,offset+4
vault b,offset+5
 */
#[derive(Clone)]
pub struct SolfiSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub sysvar_ix: AccountInfo,
    pub pool: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}
pub fn process_solfi_swap(
    bh: &Bluehouse,
    market: &SolfiSwapAccounts,
    amount: u64,
    a_to_b: bool,
) -> ProgramResult {
    let cpi_accounts = [
        &bh.base.payer,           // user
        &market.pool,             // pool
        &market.vault_a,          // vault a
        &market.vault_b,          // vault b
        &bh.base.token_a_ata,     // ata a
        &bh.base.token_a_ata,     // ata b
        &bh.base.token_a_program, // token program
        &market.sysvar_ix,        // sysvar ix
    ];
    let direction = if a_to_b { 0u8 } else { 1u8 };
    let mut instr_data = [0u8; 18];
    instr_data[0] = 7; // discriminator
    instr_data[1..9].copy_from_slice(&amount.to_le_bytes()); // amount
    instr_data[9..17].copy_from_slice(&0u64.to_le_bytes());
    instr_data[17] = direction;
    execute_cpi::<8>(
        &SOLFI_PROGRAM_ID,
        &cpi_accounts,
        &SOLFI_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
