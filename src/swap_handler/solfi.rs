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
pub fn process_solfi_swap(
    accounts: &[AccountInfo],
    amount: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let direction = if a_to_b { 0u8 } else { 1u8 };
    let mut instr_data = [0u8; 18];
    instr_data[0] = 7; // discriminator
    instr_data[1..9].copy_from_slice(&amount); // amount
    instr_data[9..17].copy_from_slice(&0u64.to_le_bytes());
    instr_data[17] = direction;

    let cpi_accounts = [
        accounts[0].clone(),          // user
        accounts[offset + 3].clone(), // pool
        accounts[offset + 4].clone(), // vault a
        accounts[offset + 5].clone(), // vault b
        accounts[offset + 2].clone(), // ata a
        accounts[offset + 8].clone(), // ata b
        accounts[3].clone(),          // token program
        accounts[offset + 2].clone(), // sysvar ix
    ];

    execute_cpi::<8>(
        &SOLFI_PROGRAM_ID,
        &cpi_accounts,
        &SOLFI_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
