use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const SOLFI_SWAP_FLAGS: &[u8] = &[2, 1, 1, 1, 1, 1, 0, 0]; //8
/*
accounts passed to solfi
program_id,offset+0
sysvar ix,offset+1
pool,offset+2
vault a,offset+3
vault b,offset+4
 */

pub fn process_solfi_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
) -> ProgramResult {
    let direction = if a_to_b { 0u8 } else { 1u8 };

    let cpi_accounts: [&AccountInfo; 8] = [
        &bh.base.payer,           // user
        &accounts[offset + 2],    // pool
        &accounts[offset + 3],    // vault a
        &accounts[offset + 4],    // vault b
        &bh.base.token_a_ata,     // user ATA a
        &bh.base.token_b_ata,     // user ATA b
        &bh.base.token_a_program, // token program
        &accounts[offset + 1],    // sysvar ix
    ];

    let mut instr_data = [0u8; 18];
    instr_data[0] = 7; // discriminator
    instr_data[1..9].copy_from_slice(&amount);
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
